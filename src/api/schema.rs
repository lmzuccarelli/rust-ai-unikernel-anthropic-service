use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicChatCompletions {
    pub model: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub role: String,
    pub content: Vec<Content>,
    #[serde(rename = "stop_reason")]
    pub stop_reason: String,
    #[serde(rename = "stop_sequence")]
    pub stop_sequence: Value,
    pub usage: Usage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "input_tokens")]
    pub input_tokens: i64,
    #[serde(rename = "cache_creation_input_tokens")]
    pub cache_creation_input_tokens: i64,
    #[serde(rename = "cache_read_input_tokens")]
    pub cache_read_input_tokens: i64,
    #[serde(rename = "cache_creation")]
    pub cache_creation: CacheCreation,
    #[serde(rename = "output_tokens")]
    pub output_tokens: i64,
    #[serde(rename = "service_tier")]
    pub service_tier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheCreation {
    #[serde(rename = "ephemeral_5m_input_tokens")]
    pub ephemeral_5m_input_tokens: i64,
    #[serde(rename = "ephemeral_1h_input_tokens")]
    pub ephemeral_1h_input_tokens: i64,
}
