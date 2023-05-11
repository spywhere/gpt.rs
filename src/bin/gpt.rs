use gpt::cli::{Parser, Cli, Commands};
use gpt::{gpt, hook};

fn main() {
  let cli: Cli = Cli::parse();

  match &cli.command {
    Commands::Gpt(opts) => gpt(opts),
    Commands::Hook => hook(),
  }
}
