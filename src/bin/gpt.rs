use gpt::cli::{Parser, Cli, Commands};
use gpt::{gpt, hook};

fn main() {
  let cli: Cli = Cli::parse();

  let result = match &cli.command {
    Commands::Gpt(opts) => gpt(opts),
    Commands::Hook => hook(),
  };

  match result {
    Err(exit) => {
      match exit.exit_code {
        0 => println!("{}", exit.message),
        _ => println!("ERROR: {}", exit.message)
      }
      std::process::exit(exit.exit_code)
    },
    _ => {},
  }
}
