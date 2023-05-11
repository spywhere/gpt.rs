use atty;
use crate::cli::options::GptOptions;
use crate::cli::Error;

pub(super) fn validate(opts: &GptOptions) -> Result<(), Error> {
  validate_api_key(opts)?;
  validate_context(opts)?;
  validate_prompt(opts)?;

  Ok(())
}

fn validate_api_key(opts: &GptOptions) -> Result<(), Error> {
  match opts.envs.api_key {
    Some(_) => Ok(()),
    None => Err(Error { exit_code: 1, message: "OPENAPI_API_KEY is required" })
  }
}

fn validate_context(opts: &GptOptions) -> Result<(), Error> {
  match (atty::is(atty::Stream::Stdout), &opts.flags.context) {
    (true, Some(_)) => Err(Error { exit_code: 1, message: "context cannot be used in the pipe" }),
    (_, _) => Ok(())
  }
}

fn validate_prompt(opts: &GptOptions) -> Result<(), Error> {
  match (&opts.flags.context, opts.prompt.join(" ").as_str()) {
    (None, "") => Err(Error { exit_code: 0, message: "No prompt given\nUse --help for usage" }),
    (_, _) => Ok(())
  }
}
