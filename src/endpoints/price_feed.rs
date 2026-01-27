//! Price feed endpoints for Parcl exchange trading data.

use crate::endpoints::market_metrics::MetricsParams;
use crate::error::{ParclError, Result};
use crate::models::{PaginatedResponse, PriceFeedEntry};
use reqwest::Client;

/// Client for price feed API endpoints.
pub struct PriceFeedClient<'a> {
    http: &'a Client,
    base_url: &'a str,
    api_key: &'a str,
}

impl<'a> PriceFeedClient<'a> {
    pub(crate) fn new(http: &'a Client, base_url: &'a str, api_key: &'a str) -> Self {
        Self {
            http,
            base_url,
            api_key,
        }
    }

    /// Retrieves historical price feed data for a market.
    pub async fn history(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<PaginatedResponse<PriceFeedEntry>> {
        let query = params.map(|p| p.to_query_string()).unwrap_or_default();
        let url = format!(
            "{}/v1/price_feed/{}/history{}",
            self.base_url, parcl_id, query
        );

        let response = self
            .http
            .get(&url)
            .header("Authorization", self.api_key)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            return Err(ParclError::ApiError {
                status: status.as_u16(),
                message,
            });
        }

        let data: PaginatedResponse<PriceFeedEntry> = response.json().await?;
        Ok(data)
    }
}
