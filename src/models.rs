use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Query {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "query")]
    pub sql: String,
    pub data_source_id: u64,
    #[serde(default)]
    pub user: Option<QueryUser>,
    pub schedule: Option<Schedule>,
    pub options: QueryOptions,
    #[serde(default)]
    pub visualizations: Vec<Visualization>,
    pub tags: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_draft: bool,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateQuery {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "query")]
    pub sql: String,
    pub data_source_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<QueryOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_draft: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryUser {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryOptions {
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameter {
    pub name: String,
    pub title: String,
    #[serde(rename = "type")]
    pub param_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(rename = "enumOptions", skip_serializing_if = "Option::is_none")]
    pub enum_options: Option<String>,
    #[serde(rename = "queryId", skip_serializing_if = "Option::is_none")]
    pub query_id: Option<u64>,
    #[serde(rename = "multiValuesOptions", skip_serializing_if = "Option::is_none")]
    pub multi_values_options: Option<MultiValuesOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiValuesOptions {
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    #[serde(rename = "quoteCharacter", skip_serializing_if = "Option::is_none")]
    pub quote_character: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Schedule {
    pub interval: Option<u64>,
    pub time: Option<String>,
    pub day_of_week: Option<String>,
    pub until: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Visualization {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub viz_type: String,
    pub options: serde_json::Value,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueriesResponse {
    pub results: Vec<Query>,
    pub count: u64,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryMetadata {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub data_source_id: u64,
    #[serde(default)]
    pub user_id: Option<u64>,
    pub schedule: Option<Schedule>,
    pub options: QueryOptions,
    pub visualizations: Vec<Visualization>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String>,
}
