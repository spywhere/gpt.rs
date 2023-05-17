use std::io;
use std::io::Write;
use std::path::Path;
use atty;

use crate::cli::options::GptOptions;
use crate::cli::Exit;

mod validation;
mod openai;

use openai::model;

pub fn gpt(opts: &GptOptions) -> Result<(), Exit> {
  validation::validate(opts)?;

  if opts.flags.show_models {
    return models(opts)
  }

  let gpt_prompt = if atty::is(atty::Stream::Stdin) {
    opts.prompt.join("")
  } else {
    io::stdin().lines()
      .map(|line| line.unwrap())
      .collect::<Vec<String>>()
      .join("\n")
  };

  validation::validate_prompt(opts, &gpt_prompt)?;
  prompt(opts, &gpt_prompt)
}

fn to_exit(error: openai::ApiError) -> Exit {
  match error.kind {
    openai::ApiErrorType::Timeout => Exit {
      exit_code: 1, message: Some("Request timed out".to_string())
    },
    openai::ApiErrorType::Decode => Exit {
      exit_code: 1, message: Some("Bad response".to_string())
    },
    openai::ApiErrorType::DryDebug => Exit { exit_code: 0, message: None },
    _ => Exit { exit_code: 1, message: Some("Request failed".to_string()) }
  }
}

pub fn models(opts: &GptOptions) -> Result<(), Exit> {
  let openai = openai::OpenAi::new(
    opts.envs.api_host.to_string(),
    opts.envs.api_key.clone().unwrap().to_string(),
    opts.flags.timeout,
    opts.debug.debug || opts.debug.debug_dry,
    opts.debug.debug_dry
  );
  let models = openai.models().map_err(to_exit)?;

  println!("Available models:");
  for model in models.models {
    println!("  - {}\n    by: {}", model.id, model.by);
  }

  Ok(())
}

pub fn prompt(opts: &GptOptions, prompt: &String) -> Result<(), Exit> {
  let mut messages: Vec<model::Message> = Vec::new();
  let mut has_context = false;

  if let Some(context) = &opts.flags.context {
    has_context = true;
    let mut skip_create = false;
    if context != "-" {
      println!("==== Context will be stored in '{}' ====", context);

      if Path::new(context).exists() {
        let data = std::fs::read_to_string(context)
          .map_err(|_| Exit {
            exit_code: 1,
            message: Some("Failed to read context file".to_string())
          })?;

        messages = serde_json::from_str(&data)
          .map_err(|_| Exit {
            exit_code: 1,
            message: Some("Failed to parse context file".to_string())
          })?;

        for message in &messages {
          match message.role {
            model::Role::System => println!("[System] {}", message.content),
            model::Role::Assistant => println!("[Assistant] {}", message.content),
            model::Role::User => println!("[User] {}", message.content)
          }
        }
        skip_create = true;
      }
    }

    if !skip_create && prompt != "" {
      println!("[System] {}", prompt);
      messages.push(model::Message {
        role: model::Role::System,
        content: prompt.to_string()
      });
    }
  }

  loop {
    if has_context {
      print!("[User] ");
      io::stdout().flush().unwrap();
      let mut line = String::new();
      if let Ok(_) = io::stdin().read_line(&mut line) {
        line = line.trim().to_string();
      }
      if line == "" {
        break;
      }
      messages.push(model::Message {
        role: model::Role::User,
        content: line
      });
    } else {
      messages.push(model::Message {
        role: model::Role::User,
        content: prompt.to_string()
      });
    }

    let request = model::ChatCompletions {
      model: opts.flags.model.clone(),
      max_tokens: opts.flags.max_tokens,
      temperature: opts.flags.temperature,
      messages: messages.clone()
    };

    let openai = openai::OpenAi::new(
      opts.envs.api_host.to_string(),
      opts.envs.api_key.clone().unwrap().to_string(),
      opts.flags.timeout,
      opts.debug.debug || opts.debug.debug_dry,
      opts.debug.debug_dry
    );
    let response = openai.chat_completions(&request).map_err(to_exit)?;

    let response = match response {
      model::ChatCompletionsResponse::Ok(response) => response,
      model::ChatCompletionsResponse::Error(error) => {
        return Err(Exit {
          exit_code: 1,
          message: Some(format!(
            "[{}] {}",
            error.error.r#type,
            error.error.message
          ).to_string())
        });
      }
    };

    let response: Vec<String> = response.choices
      .into_iter()
      .map(|choice| choice.message.content)
      .collect();

    if let Some(context) = &opts.flags.context {
      println!("[Assistant] {}", response.join("\n"));
      messages.push(model::Message {
        role: model::Role::Assistant,
        content: response.join("\n")
      });

      if context != "-" {
        std::fs::write(context, serde_json::to_string(&messages).unwrap())
          .map_err(|_| Exit {
            exit_code: 1,
            message: Some("Failed to write context file".to_string())
          })?;
      }
    } else {
      println!("{}", response.join("\n"));
      break;
    }
  }

  Ok(())
}
