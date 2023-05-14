use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Models {
  #[serde(rename = "data")]
  pub models: Vec<Model>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Model {
  pub id: String,
  #[serde(rename = "owned_by")]
  pub by: String
}

#[derive(Serialize)]
pub struct ChatCompletions {
  pub model: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_tokens: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub temperature: Option<f32>,
  pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionsResponse {
  pub choices: Vec<Choice>,
  pub usage: Usage
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
  pub message: Message
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
  pub prompt_tokens: u16,
  pub completion_tokens: u16,
  pub total_tokens: u16
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
  pub role: Role,
  pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
  System,
  User,
  Assistant
}
