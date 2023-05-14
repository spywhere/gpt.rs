use atty;

use crate::cli::options::GptOptions;
use crate::cli::Exit;

pub(super) fn validate(opts: &GptOptions) -> Result<(), Exit> {
  validate_api_key(opts)?;
  validate_context(opts)?;

  Ok(())
}

fn validate_api_key(opts: &GptOptions) -> Result<(), Exit> {
  match opts.envs.api_key {
    Some(_) => Ok(()),
    None => Err(Exit { exit_code: 1, message: Some("OPENAPI_API_KEY is required".to_string()) })
  }
}

fn validate_context(opts: &GptOptions) -> Result<(), Exit> {
  match (atty::is(atty::Stream::Stdin), &opts.flags.context) {
    (false, Some(_)) => Err(Exit { exit_code: 1, message: Some("context cannot be used in the pipe".to_string()) }),
    (_, _) => Ok(())
  }
}

pub(super) fn validate_prompt(opts: &GptOptions, prompt: &String) -> Result<(), Exit> {
  match (&opts.flags.context, prompt.as_str()) {
    (None, "") => Err(Exit { exit_code: 0, message: Some("No prompt given\nUse --help for usage".to_string()) }),
    (_, _) => Ok(())
  }
}
