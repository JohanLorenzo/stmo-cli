#![allow(clippy::missing_errors_doc)]

use anyhow::{Context, Result};
use reqwest::{Client, header};
use crate::models::{CreateQuery, QueriesResponse, Query};

pub struct RedashClient {
    client: Client,
    base_url: String,
}

impl RedashClient {
    pub fn new(base_url: String, api_key: &str) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Key {api_key}"))
                .context("Invalid API key format")?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            base_url,
        })
    }

    pub async fn list_my_queries(&self, page: u32, page_size: u32) -> Result<QueriesResponse> {
        let url = format!("{}/api/queries/my?page={page}&page_size={page_size}", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch my queries")?
            .error_for_status()
            .context("API returned error status")?;

        response
            .json()
            .await
            .context("Failed to parse queries response")
    }

    pub async fn get_query(&self, query_id: u64) -> Result<Query> {
        let url = format!("{}/api/queries/{query_id}", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await
            .context(format!("Failed to fetch query {query_id}"))?
            .error_for_status()
            .context("API returned error status")?;

        response
            .json()
            .await
            .context("Failed to parse query response")
    }


    pub async fn create_query(&self, create_query: &CreateQuery) -> Result<Query> {
        let url = format!("{}/api/queries", self.base_url);
        let response = self.client
            .post(&url)
            .json(create_query)
            .send()
            .await
            .context("Failed to create query")?
            .error_for_status()
            .context("API returned error status")?;

        response
            .json()
            .await
            .context("Failed to parse query create response")
    }

    pub async fn create_or_update_query(&self, query: &Query) -> Result<Query> {
        let url = format!("{}/api/queries/{}", self.base_url, query.id);
        let response = self.client
            .post(&url)
            .json(query)
            .send()
            .await
            .context(format!("Failed to update query {}", query.id))?
            .error_for_status()
            .context("API returned error status")?;

        response
            .json()
            .await
            .context("Failed to parse query update response")
    }

    pub async fn create_visualization(&self, query_id: u64, viz: &crate::models::CreateVisualization) -> Result<crate::models::Visualization> {
        let url = format!("{}/api/visualizations", self.base_url);
        let response = self.client
            .post(&url)
            .json(viz)
            .send()
            .await
            .context(format!("Failed to create visualization for query {query_id}"))?
            .error_for_status()
            .context("API returned error status")?;

        response
            .json()
            .await
            .context("Failed to parse visualization create response")
    }

    pub async fn update_visualization(&self, viz: &crate::models::Visualization) -> Result<crate::models::Visualization> {
        let url = format!("{}/api/visualizations/{}", self.base_url, viz.id);
        let response = self.client
            .post(&url)
            .json(viz)
            .send()
            .await
            .context(format!("Failed to update visualization {}", viz.id))?
            .error_for_status()
            .context("API returned error status")?;

        response
            .json()
            .await
            .context("Failed to parse visualization update response")
    }

    pub async fn fetch_all_queries(&self) -> Result<Vec<Query>> {
        let mut all_queries = Vec::new();
        let mut page = 1;
        let page_size = 100;

        loop {
            let response = self.list_my_queries(page, page_size).await?;

            if response.results.is_empty() {
                break;
            }

            all_queries.extend(response.results);
            eprintln!("Fetched {} / {} queries...", all_queries.len(), response.count);

            #[allow(clippy::cast_possible_truncation)]
            if all_queries.len() >= response.count as usize {
                break;
            }

            page += 1;
        }

        Ok(all_queries)
    }

    pub async fn refresh_query(
        &self,
        query_id: u64,
        parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Result<crate::models::Job> {
        let url = format!("{}/api/queries/{query_id}/results", self.base_url);

        let request = crate::models::RefreshRequest {
            max_age: 0,
            parameters,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context(format!("Failed to refresh query {query_id}"))?
            .error_for_status()
            .context("API returned error status")?;

        let job_response: crate::models::JobResponse = response
            .json()
            .await
            .context("Failed to parse job response")?;

        Ok(job_response.job)
    }

    pub async fn poll_job(&self, job_id: &str) -> Result<crate::models::Job> {
        let url = format!("{}/api/jobs/{job_id}", self.base_url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .context(format!("Failed to poll job {job_id}"))?
            .error_for_status()
            .context("API returned error status")?;

        let job_response: crate::models::JobResponse = response
            .json()
            .await
            .context("Failed to parse job response")?;

        Ok(job_response.job)
    }

    pub async fn get_query_result(
        &self,
        query_id: u64,
        result_id: u64,
    ) -> Result<crate::models::QueryResult> {
        let url = format!(
            "{}/api/queries/{query_id}/results/{result_id}.json",
            self.base_url
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .context(format!("Failed to fetch result {result_id} for query {query_id}"))?
            .error_for_status()
            .context("API returned error status")?;

        let result_response: crate::models::QueryResultResponse = response
            .json()
            .await
            .context("Failed to parse query result response")?;

        Ok(result_response.query_result)
    }

    pub async fn execute_query_with_polling(
        &self,
        query_id: u64,
        parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
        timeout_secs: u64,
        poll_interval_ms: u64,
    ) -> Result<crate::models::QueryResult> {
        use tokio::time::{sleep, Duration};
        use crate::models::JobStatus;

        eprintln!("Executing query {query_id}...");
        let job = self.refresh_query(query_id, parameters).await?;

        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(timeout_secs);
        let poll_interval = Duration::from_millis(poll_interval_ms);

        let mut current_job = job;
        loop {
            if start.elapsed() > timeout {
                anyhow::bail!("Query execution timed out after {timeout_secs} seconds");
            }

            let status = JobStatus::from_u8(current_job.status)?;

            match status {
                JobStatus::Success => {
                    let result_id = current_job.query_result_id
                        .context("Job succeeded but no result_id returned")?;

                    eprintln!("Query completed, fetching results...");
                    return self.get_query_result(query_id, result_id).await;
                }
                JobStatus::Failure => {
                    let error = current_job.error.unwrap_or_else(|| "Unknown error".to_string());
                    anyhow::bail!("Query execution failed: {error}");
                }
                JobStatus::Cancelled => {
                    anyhow::bail!("Query execution was cancelled");
                }
                JobStatus::Pending | JobStatus::Started => {
                    eprint!(".");
                    sleep(poll_interval).await;
                    current_job = self.poll_job(&current_job.id).await?;
                }
            }
        }
    }
}
