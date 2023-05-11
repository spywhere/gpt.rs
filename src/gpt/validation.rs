use crate::cli::options::GptOptions;
use crate::cli::Error;

pub(super) fn validate(opts: &GptOptions) -> Result<(), Error> {
  match opts.envs.api_key {
    Some(_) => Ok(()),
    None => Err(Error { exit_code: 1, message: "OPENAPI_API_KEY is required" })
  }
}
