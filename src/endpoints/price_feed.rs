//! Price feed endpoints for Parcl exchange trading data.

use crate::endpoints::market_metrics::MetricsParams;
use crate::error::Result;
use crate::models::{BatchMetricsResponse, MetricsResponse, PriceFeedEntry};
use crate::ParclClient;

/// Client for price feed API endpoints.
pub struct PriceFeedClient<'a> {
    client: &'a ParclClient,
}

impl<'a> PriceFeedClient<'a> {
    pub(crate) fn new(client: &'a ParclClient) -> Self {
        Self { client }
    }

    /// Retrieves historical price feed data for a market.
    pub async fn history(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<MetricsResponse<PriceFeedEntry>> {
        let params = params.unwrap_or_default();
        let url = format!(
            "{}/v1/price_feed/{}/history{}",
            self.client.base_url,
            parcl_id,
            params.to_query_string()
        );
        let resp = super::common::get_with_pagination(
            &self.client.http,
            &self.client.api_key,
            &url,
            params.auto_paginate,
            &self.client.retry_config,
        )
        .await?;
        self.client.update_credits(&resp.account);
        Ok(resp)
    }

    /// Retrieves historical rental price feed data for a market.
    pub async fn rental_history(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<MetricsResponse<PriceFeedEntry>> {
        let params = params.unwrap_or_default();
        let url = format!(
            "{}/v1/price_feed/{}/rental_price_feed{}",
            self.client.base_url,
            parcl_id,
            params.to_query_string()
        );
        let resp = super::common::get_with_pagination(
            &self.client.http,
            &self.client.api_key,
            &url,
            params.auto_paginate,
            &self.client.retry_config,
        )
        .await?;
        self.client.update_credits(&resp.account);
        Ok(resp)
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
        let url = format!("{}/v1/price_feed/history", self.client.base_url);
        let resp = super::common::post_with_pagination(
            &self.client.http,
            &self.client.api_key,
            &url,
            &body,
            params.auto_paginate,
            &self.client.retry_config,
        )
        .await?;
        self.client.update_credits(&resp.account);
        Ok(resp)
    }

    /// Batch retrieves historical rental price feed data for multiple markets.
    pub async fn batch_rental_history(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<MetricsParams>,
    ) -> Result<BatchMetricsResponse<PriceFeedEntry>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/price_feed/rental_price_feed", self.client.base_url);
        let resp = super::common::post_with_pagination(
            &self.client.http,
            &self.client.api_key,
            &url,
            &body,
            params.auto_paginate,
            &self.client.retry_config,
        )
        .await?;
        self.client.update_credits(&resp.account);
        Ok(resp)
    }
}
