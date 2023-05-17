use gpt::cli::stdout;
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
      if let Some(message) = exit.message {
        match exit.exit_code {
          0 => stdout!("{}", message),
          _ => stdout!("ERROR: {}", message)
        }
      }
      std::process::exit(exit.exit_code)
    },
    _ => {},
  }
}
