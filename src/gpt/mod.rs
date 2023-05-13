mod validation;
mod openai;

use atty;
use crate::cli::options::GptOptions;
use crate::cli::Exit;

pub fn gpt(opts: &GptOptions) -> Result<(), Exit> {
  if opts.flags.show_models {
    return models(opts)
  }

  validation::validate(opts)?;

  prompt(opts)
}

fn to_exit(error: openai::ApiError) -> Exit {
  match error.kind {
    openai::ApiErrorType::Timeout => Exit { exit_code: 1, message: "Request timed out".to_string() },
    openai::ApiErrorType::Decode => Exit { exit_code: 1, message: "Bad response".to_string() },
    _ => Exit { exit_code: 1, message: "Request failed".to_string() }
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

pub fn prompt(opts: &GptOptions) -> Result<(), Exit> {
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
  println!("Prompt: {}", opts.prompt.join(" "));

  println!("Produce Command: {}", opts.helpers.produce_command);

  println!("Is TTY: {}", atty::is(atty::Stream::Stdin));

  let openai = openai::OpenAi::new(
    opts.envs.api_host.to_string(),
    opts.envs.api_key.clone().unwrap().to_string(),
    opts.flags.timeout,
    opts.debug || opts.debug_dry,
    opts.debug_dry
  );
  let models = openai.models().map_err(to_exit)?;

  Ok(())
}
