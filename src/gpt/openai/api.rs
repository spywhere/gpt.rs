use std::time::Duration;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::gpt::openai::model;

pub(super) struct ApiOptions {
  pub(super) api_host: String,
  pub(super) api_key: String,
  pub(super) timeout: u8,
  pub(super) debug: bool,
  pub(super) dry: bool
}

#[derive(Debug)]
pub enum ApiErrorType {
  Status(u16),
  Timeout,
  Redirect,
  Request,
  Connect,
  Body,
  Decode,
  Builder,
  Unknown
}

pub struct ApiError {
  pub kind: ApiErrorType
}

fn to_api_error(error: reqwest::Error) -> ApiError {
  match (error.is_builder(), error.is_redirect(), error.is_status(), error.is_timeout(), error.is_request(), error.is_connect(), error.is_body(), error.is_decode(), error.status()) {
    (true, _, _, _, _, _, _, _, _) => ApiError { kind: ApiErrorType::Builder },
    (_, true, _, _, _, _, _, _, _) => ApiError { kind: ApiErrorType::Redirect },
    (_, _, true, _, _, _, _, _, Some(code)) => ApiError { kind: ApiErrorType::Status(code.as_u16()) },
    (_, _, _, true, _, _, _, _, _) => ApiError { kind: ApiErrorType::Timeout },
    (_, _, _, _, true, _, _, _, _) => ApiError { kind: ApiErrorType::Request },
    (_, _, _, _, _, true, _, _, _) => ApiError { kind: ApiErrorType::Connect },
    (_, _, _, _, _, _, true, _, _) => ApiError { kind: ApiErrorType::Body },
    (_, _, _, _, _, _, _, true, _) => ApiError { kind: ApiErrorType::Decode },
    _ => ApiError { kind: ApiErrorType::Unknown },
  }
}

fn get<T: DeserializeOwned>(opts: &ApiOptions, path: impl Into<String>) -> Result<T, ApiError> {
  let path = path.into().clone();
  println!("Req[{}{}]", opts.api_host, path);

  let authorization = reqwest::header::HeaderValue::from_str(format!("Bearer {}", opts.api_key).as_str()).map_err(|_| ApiError { kind: ApiErrorType::Request })?;
  let client = reqwest::blocking::Client::new();
  client
    .get(format!("{}{}", opts.api_host, path))
    .timeout(Duration::from_secs(opts.timeout.into()))
    .header(reqwest::header::AUTHORIZATION, authorization)
    .send().map_err(to_api_error)?
    .json().map_err(to_api_error)
}

fn post<S: Serialize, T: DeserializeOwned>(opts: &ApiOptions, path: impl Into<String>, json: &S) -> Result<T, ApiError> {
  let path = path.into().clone();
  println!("Req[{}{}]", opts.api_host, path);

  let authorization = reqwest::header::HeaderValue::from_str(format!("Bearer {}", opts.api_key).as_str()).map_err(|_| ApiError { kind: ApiErrorType::Request })?;
  let client = reqwest::blocking::Client::new();
  client
    .post(format!("{}{}", opts.api_host, path))
    .timeout(Duration::from_secs(opts.timeout.into()))
    .header(reqwest::header::AUTHORIZATION, authorization)
    .json(&json)
    .send().map_err(to_api_error)?
    .json().map_err(to_api_error)
}

pub(super) fn models(opts: &ApiOptions) -> Result<model::Models, ApiError> {
  get(opts, "/models")
}

pub(super) fn chat_completions(opts: &ApiOptions, request: &model::ChatCompletions) -> Result<model::ChatCompletions, ApiError> {
  post(opts, "/chat/completions", &request)
}
