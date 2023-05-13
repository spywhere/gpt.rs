mod api;
mod model;

pub use api::{ ApiError, ApiErrorType };

pub struct OpenAi {
  opts: api::ApiOptions
}

impl OpenAi {
  pub fn new(api_host: impl Into<String>, api_key: impl Into<String>, timeout: u8, debug: bool, dry: bool) -> OpenAi {
    OpenAi { opts: api::ApiOptions { api_host: api_host.into(), api_key: api_key.into(), timeout, debug, dry } }
  }

  pub fn models(&self) -> Result<model::Models, api::ApiError> {
    api::models(&self.opts)
  }

  pub fn chat_completions(&self, request: &model::ChatCompletions) -> Result<model::ChatCompletions, api::ApiError> {
    api::chat_completions(&self.opts, &request)
  }
}
