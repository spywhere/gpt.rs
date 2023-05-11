use crate::cli::Error;

pub fn hook() -> Result<(), Error> {
  println!("Generate commit message");

  Ok(())
}
