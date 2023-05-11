use gpt::cli::{Parser, Cli, Commands};
use gpt::{gpt, hook};

fn main() {
  let cli: Cli = Cli::parse();

  let result = match &cli.command {
    Commands::Gpt(opts) => gpt(opts),
    Commands::Hook => hook(),
  };

  match result {
    Err(error) => {
      println!("ERROR: {}", error.message);
      std::process::exit(error.exit_code)
    },
    _ => {},
  }
}
