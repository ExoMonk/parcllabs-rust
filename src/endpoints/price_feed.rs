//! Price feed endpoints for Parcl exchange trading data.

use crate::endpoints::market_metrics::MetricsParams;
use crate::error::{ParclError, Result};
use crate::models::{BatchMetricsResponse, MetricsResponse, PriceFeedEntry};
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
    ) -> Result<MetricsResponse<PriceFeedEntry>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/price_feed/{}/history{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves historical rental price feed data for a market.
    pub async fn rental_history(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<MetricsResponse<PriceFeedEntry>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/price_feed/{}/rental_price_feed{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    // --- Batch POST methods ---

    /// Batch retrieves historical price feed data for multiple markets.
    pub async fn batch_history(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<MetricsParams>,
    ) -> Result<BatchMetricsResponse<PriceFeedEntry>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/price_feed/history", self.base_url);
        super::common::post_with_pagination(
            self.http,
            self.api_key,
            &url,
            &body,
            params.auto_paginate,
        )
        .await
    }

    /// Batch retrieves historical rental price feed data for multiple markets.
    pub async fn batch_rental_history(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<MetricsParams>,
    ) -> Result<BatchMetricsResponse<PriceFeedEntry>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/price_feed/rental_price_feed", self.base_url);
        super::common::post_with_pagination(
            self.http,
            self.api_key,
            &url,
            &body,
            params.auto_paginate,
        )
        .await
    }

    async fn fetch_with_pagination(
        &self,
        url: &str,
        auto_paginate: bool,
    ) -> Result<MetricsResponse<PriceFeedEntry>> {
        let mut response = self.fetch_page(url).await?;

        if auto_paginate {
            while let Some(ref next_url) = response.links.next {
                let next_page = self.fetch_page(next_url).await?;
                response.items.extend(next_page.items);
                response.links = next_page.links;
            }
        }

        Ok(response)
    }

    async fn fetch_page(&self, url: &str) -> Result<MetricsResponse<PriceFeedEntry>> {
        let response = self
            .http
            .get(url)
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

        let data: MetricsResponse<PriceFeedEntry> = response.json().await?;
        Ok(data)
    }
}
