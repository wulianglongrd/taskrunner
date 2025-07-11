use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    pub name: String,
    pub commands: Vec<String>,
    pub extract: Option<Vec<ExtractRule>>,
}

#[derive(Debug, Deserialize)]
pub struct ExtractRule {
    pub field: String,

    #[serde(rename = "as")]
    pub as_key: String,
}