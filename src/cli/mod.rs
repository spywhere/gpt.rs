pub use clap::Parser;

pub mod options;

mod commands;

pub use commands::Commands;

#[derive(Parser)]
#[command(version, propagate_version = true, multicall = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}
