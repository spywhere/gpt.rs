use std::io;
use std::io::Write;
use std::path::Path;
use atty;

use crate::cli::options::GptOptions;
use crate::cli::Exit;

mod validation;
mod openai;

pub fn gpt(opts: &GptOptions) -> Result<(), Exit> {
  validation::validate(opts)?;

  if opts.flags.show_models {
    return models(opts)
  }

  let gpt_prompt = if atty::is(atty::Stream::Stdin) {
    opts.prompt.join("")
  } else {
    io::stdin().lines().map(|line| line.unwrap()).collect::<Vec<String>>().join("\n")
  };

  validation::validate_prompt(opts, &gpt_prompt)?;
  prompt(opts, &gpt_prompt)
}

fn to_exit(error: openai::ApiError) -> Exit {
  match error.kind {
    openai::ApiErrorType::Timeout => Exit { exit_code: 1, message: Some("Request timed out".to_string()) },
    openai::ApiErrorType::Decode => Exit { exit_code: 1, message: Some("Bad response".to_string()) },
    openai::ApiErrorType::DryDebug => Exit { exit_code: 0, message: None },
    _ => Exit { exit_code: 1, message: Some("Request failed".to_string()) }
  }
}

pub fn models(opts: &GptOptions) -> Result<(), Exit> {
  let openai = openai::OpenAi::new(
    opts.envs.api_host.to_string(),
    opts.envs.api_key.clone().unwrap().to_string(),
    opts.flags.timeout,
    opts.debug || opts.debug_dry,
    opts.debug_dry
  );
  let models = openai.models().map_err(to_exit)?;

  println!("Available models:");
  for model in models.models {
    println!("  - {}\n    by: {}", model.id, model.by);
  }

  Ok(())
}

pub fn prompt(opts: &GptOptions, prompt: &String) -> Result<(), Exit> {
  if let Some(temp) = opts.flags.temperature {
    println!("Temperature: {}", temp);
  }
  if let Some(token) = opts.flags.max_tokens {
    println!("Max Tokens: {}", token);
  }
  println!("Model: {}", opts.flags.model);
  if let Some(context) = &opts.flags.context {
    println!("Context: {}", context);
  }
  println!("Timeout: {}", opts.flags.timeout);

  println!("Produce Command: {}", opts.helpers.produce_command);

  println!("Is TTY: {}", atty::is(atty::Stream::Stdin));

  println!("Prompt: [{}]", prompt);

  let mut messages: Vec<openai::model::Message> = Vec::new();

  if let Some(context) = &opts.flags.context {
    let mut skip_create = false;
    if context != "-" {
      println!("==== Context will be stored in '{}' ====", context);

      if Path::new(context).exists() {
        println!("Load existing context");
        skip_create = true;
      }
    }

    if !skip_create && prompt != "" {
      println!("[System] {}", prompt);
      messages.push(openai::model::Message {
        role: openai::model::Role::System,
        content: prompt.to_string()
      });
    }
  }

  if let Some(_) = &opts.flags.context {
    print!("[User] ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    if let Ok(_) = io::stdin().read_line(&mut line) {
      line = line.trim().to_string();
    }
    if line == "" {
      return Err(Exit { exit_code: 0, message: None });
    }
    messages.push(openai::model::Message {
      role: openai::model::Role::User,
      content: line
    });
  } else {
    messages.push(openai::model::Message {
      role: openai::model::Role::User,
      content: prompt.to_string()
    });
  }

  let request = openai::model::ChatCompletions {
    model: opts.flags.model.clone(),
    max_tokens: opts.flags.max_tokens,
    temperature: opts.flags.temperature,
    messages
  };

  let openai = openai::OpenAi::new(
    opts.envs.api_host.to_string(),
    opts.envs.api_key.clone().unwrap().to_string(),
    opts.flags.timeout,
    opts.debug || opts.debug_dry,
    opts.debug_dry
  );
  let response = openai.chat_completions(&request).map_err(to_exit)?;

  let messages: Vec<String> = response.choices
    .into_iter()
    .map(|choice| choice.message.content)
    .collect();

  println!("{}", messages.join("\n"));

  Ok(())
}
