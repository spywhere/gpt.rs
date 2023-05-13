use serde::{ Serialize, Deserialize };

#[derive(Deserialize, Clone)]
pub struct Models {
  #[serde(rename = "data")]
  pub models: Vec<Model>
}

#[derive(Deserialize, Clone)]
pub struct Model {
  pub id: String,
  #[serde(rename = "owned_by")]
  pub by: String
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletions {
  #[serde(skip_deserializing)]
  pub model: String,
  #[serde(skip_deserializing)]
  pub max_tokens: Option<u16>,
  #[serde(skip_deserializing)]
  pub temperature: Option<f32>,
  #[serde(skip_deserializing)]
  pub messages: Vec<Message>,

  #[serde(skip_serializing)]
  pub choices: Vec<Choice>,
  #[serde(skip_serializing)]
  pub usage: Usage
}

#[derive(Deserialize)]
pub struct Choice {
  message: Message
}

#[derive(Deserialize)]
pub struct Usage {
  prompt_tokens: u16,
  completion_tokens: u16,
  total_tokens: u16
}

#[derive(Serialize, Deserialize)]
pub struct Message {
  pub role: Role,
  pub content: String,
}

#[derive(Serialize, Deserialize)]
pub enum Role {
  System,
  User,
  Assistant
}
