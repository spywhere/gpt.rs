use clap::Args;

#[derive(Args)]
pub struct GptOptions {
  /// Flags
  #[command(flatten)]
  pub flags: Flags,

  // Environment variables
  #[command(flatten, next_help_heading = "Environment Variables")]
  pub envs: Envs,

  /// Prompt helpers
  #[command(flatten, next_help_heading = "Prompt Helper")]
  pub helpers: Helpers,

  /// Prompt
  #[arg(hide_short_help = true, hide_long_help = true, trailing_var_arg = true)]
  pub prompt: Vec<String>,

  /// Debug options
  #[command(flatten)]
  pub debug: DebugOptions,
}

#[derive(Args)]
#[group(multiple = false)]
pub struct DebugOptions {
  #[arg(hide = true, long)]
  pub debug: bool,

  #[arg(hide = true, long)]
  pub debug_dry: bool,
}

#[derive(Args)]
pub struct Envs {
  /// API key for the given OpenAI base
  #[arg(hide_env_values = true, long = "openai-api-key", env = "OPENAI_API_KEY")]
  pub api_key: Option<String>,

  /// Host for Open API
  #[arg(hide_env_values = true, long = "openai-api-host", env = "OPENAI_API_BASE", default_value = "https://api.openai.com/v1")]
  pub api_host: String,
}

#[derive(Args)]
pub struct Flags {
  ///Sampling tempurature, higher is more random
  #[arg(short = 'T', long, value_name = "temp")]
  pub temperature: Option<f32>,

  /// Maximum number of tokens to generate in the completion
  #[arg(short = 't', long, value_name = "number")]
  pub max_tokens: Option<u16>,

  /// ID of the model to use
  #[arg(short, long, default_value = "gpt-3.5-turbo", value_name = "name")]
  pub model: String,

  /// Use chat context, optionally use and store the content in the given file. The prompt from
  /// command line will be passed as a system message to the assisteant, cannot be used in a pipe
  #[arg(short, long, next_line_help = true, value_name = "file")]
  pub context: Option<String>,

  /// Maximum number of seconds before request timed out
  #[arg(long, default_value_t = 30, value_name = "number")]
  pub timeout: u8,

  /// List all available models
  #[arg(long = "models")]
  pub show_models: bool,
}

#[derive(Args)]
#[group(multiple = false)]
pub struct Helpers {
  /// Produce only shell command output
  #[arg(id = "command", visible_alias = "cmd", long = "command")]
  pub produce_command: bool,

  /// Produce only one-liner code
  #[arg(id = "code", long = "code")]
  pub produce_code: bool,

  /// Produce only URL for the given query
  #[arg(id = "url", long = "url")]
  pub produce_url: bool,

  /// Produce a copilot-like shell command with explanation
  #[arg(hide = true, id = "copilot", long = "copilot")]
  pub produce_copilot: bool,
}
