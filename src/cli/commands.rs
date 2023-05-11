use clap::Subcommand;

use crate::cli::options::GptOptions;

#[derive(Subcommand)]
pub enum Commands {
  #[command(name = "gpt")]
  Gpt(GptOptions),
  #[command(name = "prepare-commit-msg")]
  Hook,
}
