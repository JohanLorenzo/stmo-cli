use anyhow::{Context, Result};
use reqwest::{Client, header};
use crate::models::{Dashboard, QueriesResponse, Query};

#[derive(serde::Deserialize)]
struct FavoritesResponse {
    results: Vec<crate::models::DashboardSummary>,
}

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

    pub async fn get_dashboard(&self, slug: &str) -> Result<Dashboard> {
        let url = format!("{}/api/dashboards/{slug}", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await
            .context(format!("Failed to fetch dashboard {slug}"))?
            .error_for_status()
            .context("API returned error status")?;

        response
            .json()
            .await
            .context("Failed to parse dashboard response")
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

    pub async fn fetch_my_dashboard_summaries(&self) -> Result<Vec<crate::models::DashboardSummary>> {
        let url = format!("{}/api/dashboards/favorites", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch favorite dashboards")?
            .error_for_status()
            .context("API returned error status")?;

        let fav_response: FavoritesResponse = response
            .json()
            .await
            .context("Failed to parse favorites response")?;

        Ok(fav_response.results)
    }
}
