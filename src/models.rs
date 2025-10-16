use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dashboard {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub user_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    pub layout: Vec<Vec<u64>>,
    #[serde(rename = "dashboard_filters_enabled")]
    pub filters_enabled: bool,
    pub widgets: Vec<Widget>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub is_draft: bool,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub version: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Widget {
    pub id: u64,
    pub width: u64,
    pub options: WidgetOptions,
    pub dashboard_id: u64,
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visualization: Option<WidgetVisualization>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetOptions {
    #[serde(rename = "isHidden", default)]
    pub is_hidden: bool,
    pub position: Position,
    #[serde(rename = "parameterMappings", default)]
    pub parameter_mappings: HashMap<String, ParameterMapping>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    #[serde(rename = "autoHeight", default)]
    pub auto_height: bool,
    #[serde(rename = "sizeX")]
    pub size_x: u64,
    #[serde(rename = "sizeY")]
    pub size_y: u64,
    #[serde(rename = "minSizeX", default)]
    pub min_size_x: u64,
    #[serde(rename = "maxSizeX", default)]
    pub max_size_x: u64,
    #[serde(rename = "minSizeY", default)]
    pub min_size_y: u64,
    #[serde(rename = "maxSizeY", default)]
    pub max_size_y: u64,
    pub col: u64,
    pub row: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterMapping {
    pub name: String,
    #[serde(rename = "type")]
    pub mapping_type: String,
    #[serde(rename = "mapTo")]
    pub map_to: String,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetVisualization {
    pub id: u64,
    pub query: WidgetQuery,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetQuery {
    pub id: u64,
    pub name: String,
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
pub struct DashboardsResponse {
    pub results: Vec<DashboardSummary>,
    pub count: u64,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DashboardSummary {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub tags: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_draft: bool,
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
