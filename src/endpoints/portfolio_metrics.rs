//! Portfolio metrics endpoints for tracking portfolio-level housing data.

use crate::error::{ParclError, Result};
use crate::models::{
    BatchMetricsResponse, MetricsResponse, PortfolioHousingEventCounts,
    PortfolioNewListingsRollingCounts, PortfolioRentalListingsRollingCounts, PortfolioSize,
    PortfolioStockOwnership,
};
use reqwest::Client;

/// Client for portfolio metrics API endpoints.
pub struct PortfolioMetricsClient<'a> {
    http: &'a Client,
    base_url: &'a str,
    api_key: &'a str,
}

/// Query parameters for portfolio metrics requests.
#[derive(Debug, Default, Clone)]
pub struct PortfolioMetricsParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub portfolio_size: Option<PortfolioSize>,
    pub auto_paginate: bool,
}

impl PortfolioMetricsParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Maximum number of results per page.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Offset for pagination.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Filter results starting from this date (YYYY-MM-DD).
    pub fn start_date(mut self, date: impl Into<String>) -> Self {
        self.start_date = Some(date.into());
        self
    }

    /// Filter results ending at this date (YYYY-MM-DD).
    pub fn end_date(mut self, date: impl Into<String>) -> Self {
        self.end_date = Some(date.into());
        self
    }

    /// Filter by portfolio size category.
    pub fn portfolio_size(mut self, portfolio_size: PortfolioSize) -> Self {
        self.portfolio_size = Some(portfolio_size);
        self
    }

    /// Automatically fetch all pages of results.
    pub fn auto_paginate(mut self, auto_paginate: bool) -> Self {
        self.auto_paginate = auto_paginate;
        self
    }

    pub(crate) fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(l) = self.limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = self.offset {
            params.push(format!("offset={}", o));
        }
        if let Some(ref s) = self.start_date {
            params.push(format!("start_date={}", s));
        }
        if let Some(ref e) = self.end_date {
            params.push(format!("end_date={}", e));
        }
        if let Some(ps) = self.portfolio_size {
            params.push(format!("portfolio_size={}", ps.as_str()));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }

    pub(crate) fn to_batch_body(&self, parcl_ids: &[i64]) -> serde_json::Value {
        let mut body = serde_json::json!({ "parcl_id": parcl_ids });
        let obj = body.as_object_mut().unwrap();
        if let Some(l) = self.limit {
            obj.insert("limit".into(), serde_json::json!(l));
        }
        if let Some(o) = self.offset {
            obj.insert("offset".into(), serde_json::json!(o));
        }
        if let Some(ref s) = self.start_date {
            obj.insert("start_date".into(), serde_json::json!(s));
        }
        if let Some(ref e) = self.end_date {
            obj.insert("end_date".into(), serde_json::json!(e));
        }
        if let Some(ps) = self.portfolio_size {
            obj.insert("portfolio_size".into(), serde_json::json!(ps.as_str()));
        }
        body
    }
}

impl<'a> PortfolioMetricsClient<'a> {
    pub(crate) fn new(http: &'a Client, base_url: &'a str, api_key: &'a str) -> Self {
        Self {
            http,
            base_url,
            api_key,
        }
    }

    /// Retrieves single-family housing stock ownership by portfolio holders.
    pub async fn sf_housing_stock_ownership(
        &self,
        parcl_id: i64,
        params: Option<PortfolioMetricsParams>,
    ) -> Result<MetricsResponse<PortfolioStockOwnership>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/portfolio_metrics/{}/sf_housing_stock_ownership{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves single-family housing event counts by portfolio holders.
    pub async fn sf_housing_event_counts(
        &self,
        parcl_id: i64,
        params: Option<PortfolioMetricsParams>,
    ) -> Result<MetricsResponse<PortfolioHousingEventCounts>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/portfolio_metrics/{}/sf_housing_event_counts{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves rolling counts of new for-sale listings by portfolio holders.
    pub async fn sf_new_listings_for_sale_rolling_counts(
        &self,
        parcl_id: i64,
        params: Option<PortfolioMetricsParams>,
    ) -> Result<MetricsResponse<PortfolioNewListingsRollingCounts>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/portfolio_metrics/{}/sf_new_listings_for_sale_rolling_counts{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves rolling counts of new rental listings by portfolio holders.
    pub async fn sf_new_listings_for_rent_rolling_counts(
        &self,
        parcl_id: i64,
        params: Option<PortfolioMetricsParams>,
    ) -> Result<MetricsResponse<PortfolioRentalListingsRollingCounts>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/portfolio_metrics/{}/sf_new_listings_for_rent_rolling_counts{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    // --- Batch POST methods ---

    /// Batch retrieves single-family housing stock ownership for multiple markets.
    pub async fn batch_sf_housing_stock_ownership(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<PortfolioMetricsParams>,
    ) -> Result<BatchMetricsResponse<PortfolioStockOwnership>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/portfolio_metrics/sf_housing_stock_ownership", self.base_url);
        super::common::post_with_pagination(
            self.http,
            self.api_key,
            &url,
            &body,
            params.auto_paginate,
        )
        .await
    }

    /// Batch retrieves single-family housing event counts for multiple markets.
    pub async fn batch_sf_housing_event_counts(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<PortfolioMetricsParams>,
    ) -> Result<BatchMetricsResponse<PortfolioHousingEventCounts>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/portfolio_metrics/sf_housing_event_counts", self.base_url);
        super::common::post_with_pagination(
            self.http,
            self.api_key,
            &url,
            &body,
            params.auto_paginate,
        )
        .await
    }

    /// Batch retrieves new for-sale listing rolling counts for multiple markets.
    pub async fn batch_sf_new_listings_for_sale_rolling_counts(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<PortfolioMetricsParams>,
    ) -> Result<BatchMetricsResponse<PortfolioNewListingsRollingCounts>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!(
            "{}/v1/portfolio_metrics/sf_new_listings_for_sale_rolling_counts",
            self.base_url
        );
        super::common::post_with_pagination(
            self.http,
            self.api_key,
            &url,
            &body,
            params.auto_paginate,
        )
        .await
    }

    /// Batch retrieves new rental listing rolling counts for multiple markets.
    pub async fn batch_sf_new_listings_for_rent_rolling_counts(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<PortfolioMetricsParams>,
    ) -> Result<BatchMetricsResponse<PortfolioRentalListingsRollingCounts>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!(
            "{}/v1/portfolio_metrics/sf_new_listings_for_rent_rolling_counts",
            self.base_url
        );
        super::common::post_with_pagination(
            self.http,
            self.api_key,
            &url,
            &body,
            params.auto_paginate,
        )
        .await
    }

    async fn fetch_with_pagination<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        auto_paginate: bool,
    ) -> Result<MetricsResponse<T>> {
        let mut response = self.fetch_page(url).await?;

        if auto_paginate {
            while let Some(ref next_url) = response.links.next {
                let next_page: MetricsResponse<T> = self.fetch_page(next_url).await?;
                response.items.extend(next_page.items);
                response.links = next_page.links;
            }
        }

        Ok(response)
    }

    async fn fetch_page<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<MetricsResponse<T>> {
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

        let data: MetricsResponse<T> = response.json().await?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PortfolioSize;

    #[test]
    fn portfolio_params_default() {
        let params = PortfolioMetricsParams::new();
        assert!(params.limit.is_none());
        assert!(params.offset.is_none());
        assert!(params.start_date.is_none());
        assert!(params.end_date.is_none());
        assert!(params.portfolio_size.is_none());
        assert!(!params.auto_paginate);
    }

    #[test]
    fn portfolio_params_builder() {
        let params = PortfolioMetricsParams::new()
            .limit(10)
            .offset(20)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .portfolio_size(PortfolioSize::Portfolio10To99)
            .auto_paginate(true);

        assert_eq!(params.limit, Some(10));
        assert_eq!(params.offset, Some(20));
        assert_eq!(params.start_date, Some("2024-01-01".into()));
        assert_eq!(params.end_date, Some("2024-12-31".into()));
        assert_eq!(
            params.portfolio_size,
            Some(PortfolioSize::Portfolio10To99)
        );
        assert!(params.auto_paginate);
    }

    #[test]
    fn portfolio_params_empty_query_string() {
        let params = PortfolioMetricsParams::new();
        assert_eq!(params.to_query_string(), "");
    }

    #[test]
    fn portfolio_params_query_string_all_fields() {
        let params = PortfolioMetricsParams::new()
            .limit(10)
            .offset(5)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .portfolio_size(PortfolioSize::Portfolio1000Plus);

        let qs = params.to_query_string();
        assert!(qs.starts_with('?'));
        assert!(qs.contains("limit=10"));
        assert!(qs.contains("offset=5"));
        assert!(qs.contains("start_date=2024-01-01"));
        assert!(qs.contains("end_date=2024-12-31"));
        assert!(qs.contains("portfolio_size=PORTFOLIO_1000_PLUS"));
    }

    #[test]
    fn portfolio_params_query_string_portfolio_size_only() {
        let params =
            PortfolioMetricsParams::new().portfolio_size(PortfolioSize::Portfolio2To9);
        assert_eq!(
            params.to_query_string(),
            "?portfolio_size=PORTFOLIO_2_TO_9"
        );
    }

    #[test]
    fn portfolio_params_auto_paginate_not_in_query() {
        let params = PortfolioMetricsParams::new()
            .limit(5)
            .auto_paginate(true);
        let qs = params.to_query_string();
        assert!(!qs.contains("auto_paginate"));
        assert!(qs.contains("limit=5"));
    }

    #[test]
    fn portfolio_params_batch_body_minimal() {
        let params = PortfolioMetricsParams::new();
        let body = params.to_batch_body(&[100, 200]);
        let obj = body.as_object().unwrap();
        assert_eq!(obj["parcl_id"], serde_json::json!([100, 200]));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("portfolio_size"));
    }

    #[test]
    fn portfolio_params_batch_body_all_fields() {
        let params = PortfolioMetricsParams::new()
            .limit(10)
            .offset(5)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .portfolio_size(PortfolioSize::Portfolio10To99);
        let body = params.to_batch_body(&[100]);
        let obj = body.as_object().unwrap();
        assert_eq!(obj["parcl_id"], serde_json::json!([100]));
        assert_eq!(obj["limit"], 10);
        assert_eq!(obj["offset"], 5);
        assert_eq!(obj["start_date"], "2024-01-01");
        assert_eq!(obj["end_date"], "2024-12-31");
        assert_eq!(obj["portfolio_size"], "PORTFOLIO_10_TO_99");
    }
}
