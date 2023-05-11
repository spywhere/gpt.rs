use crate::cli::Exit;

pub fn hook() -> Result<(), Exit> {
  println!("Generate commit message");

  Ok(())
}
