use crate::cli::options::GptOptions;

pub fn gpt(opts: &GptOptions) {
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

  println!("Models: {}", opts.flags.show_models);

  println!("Produce Command: {}", opts.helpers.produce_command);
}
