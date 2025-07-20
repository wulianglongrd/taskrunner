use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum OutputType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "delim")]
    Separated,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ExtractType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "json")]
    Json,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtractRule {
    pub as_key: String,
    pub field: String, // 支持 jsonpath
    pub extract_type: Option<ExtractType>,
} 