use crate::cli::stdout;
use crate::cli::Exit;

pub fn hook() -> Result<(), Exit> {
  stdout!("Generate commit message");

  Ok(())
}
