#![allow(clippy::missing_errors_doc)]

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

#[derive(Debug, Serialize, Clone)]
pub struct CreateVisualization {
    pub query_id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub viz_type: String,
    pub options: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub max_age: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobResponse {
    pub job: Job,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub status: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_result_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResultResponse {
    pub query_result: QueryResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub id: u64,
    pub data: QueryResultData,
    pub runtime: f64,
    pub retrieved_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResultData {
    pub columns: Vec<Column>,
    pub rows: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum JobStatus {
    Pending = 1,
    Started = 2,
    Success = 3,
    Failure = 4,
    Cancelled = 5,
}

impl JobStatus {
    pub fn from_u8(status: u8) -> anyhow::Result<Self> {
        match status {
            1 => Ok(Self::Pending),
            2 => Ok(Self::Started),
            3 => Ok(Self::Success),
            4 => Ok(Self::Failure),
            5 => Ok(Self::Cancelled),
            _ => Err(anyhow::anyhow!("Invalid job status: {status}")),
        }
    }
}

#[cfg(test)]
#[allow(clippy::missing_errors_doc)]
#[allow(clippy::unnecessary_literal_unwrap)]
mod tests {
    use super::*;

    #[test]
    fn test_job_status_from_u8_valid() {
        assert!(matches!(JobStatus::from_u8(1).unwrap(), JobStatus::Pending));
        assert!(matches!(JobStatus::from_u8(2).unwrap(), JobStatus::Started));
        assert!(matches!(JobStatus::from_u8(3).unwrap(), JobStatus::Success));
        assert!(matches!(JobStatus::from_u8(4).unwrap(), JobStatus::Failure));
        assert!(matches!(JobStatus::from_u8(5).unwrap(), JobStatus::Cancelled));
    }

    #[test]
    fn test_job_status_from_u8_invalid() {
        assert!(JobStatus::from_u8(0).is_err());
        assert!(JobStatus::from_u8(6).is_err());
        assert!(JobStatus::from_u8(255).is_err());

        let err = JobStatus::from_u8(10).unwrap_err();
        assert!(err.to_string().contains("Invalid job status"));
    }

    #[test]
    fn test_query_serialization() {
        let query = Query {
            id: 1,
            name: "Test Query".to_string(),
            description: None,
            sql: "SELECT * FROM table".to_string(),
            data_source_id: 63,
            user: None,
            schedule: None,
            options: QueryOptions { parameters: vec![] },
            visualizations: vec![],
            tags: None,
            is_archived: false,
            is_draft: false,
            updated_at: "2026-01-21".to_string(),
            created_at: "2026-01-21".to_string(),
        };

        let json = serde_json::to_string(&query).unwrap();
        assert!(json.contains("\"query\":"));
        assert!(json.contains("SELECT * FROM table"));
    }

    #[test]
    fn test_query_metadata_deserialization() {
        let yaml = r"
id: 100064
name: Test Query
description: null
data_source_id: 63
user_id: 530
schedule: null
options:
  parameters:
    - name: project
      title: project
      type: enum
      value:
        - try
      enumOptions: |
        try
        autoland
visualizations: []
tags:
  - bug 1840828
";

        let metadata: QueryMetadata = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(metadata.id, 100_064);
        assert_eq!(metadata.name, "Test Query");
        assert_eq!(metadata.data_source_id, 63);
        assert_eq!(metadata.options.parameters.len(), 1);
        assert_eq!(metadata.options.parameters[0].name, "project");
    }
}
