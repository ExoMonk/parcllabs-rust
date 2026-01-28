//! Market metrics endpoints for housing data retrieval.

use crate::error::{ParclError, Result};
use crate::models::{
    AllCash, HousingEventCounts, HousingEventPrices, HousingEventPropertyAttributes, HousingStock,
    MetricsResponse, PropertyType,
};
use reqwest::Client;

/// Client for market metrics API endpoints.
pub struct MarketMetricsClient<'a> {
    http: &'a Client,
    base_url: &'a str,
    api_key: &'a str,
}

/// Query parameters for paginated metrics requests.
#[derive(Debug, Default, Clone)]
pub struct MetricsParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub property_type: Option<PropertyType>,
    pub auto_paginate: bool,
}

impl MetricsParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Maximum number of results per page
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Offset for pagination
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Filter results starting from this date (YYYY-MM-DD)
    pub fn start_date(mut self, date: impl Into<String>) -> Self {
        self.start_date = Some(date.into());
        self
    }

    /// Filter results ending at this date (YYYY-MM-DD)
    pub fn end_date(mut self, date: impl Into<String>) -> Self {
        self.end_date = Some(date.into());
        self
    }

    /// Filter by property type (single family, condo, townhouse, etc.)
    pub fn property_type(mut self, property_type: PropertyType) -> Self {
        self.property_type = Some(property_type);
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
        if let Some(pt) = self.property_type {
            params.push(format!("property_type={}", pt.as_str()));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

impl<'a> MarketMetricsClient<'a> {
    pub(crate) fn new(http: &'a Client, base_url: &'a str, api_key: &'a str) -> Self {
        Self {
            http,
            base_url,
            api_key,
        }
    }

    /// Retrieves housing event counts (sales, listings) for a market.
    pub async fn housing_event_counts(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<MetricsResponse<HousingEventCounts>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/market_metrics/{}/housing_event_counts{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves housing stock data (single-family, condo, townhouse counts).
    pub async fn housing_stock(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<MetricsResponse<HousingStock>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/market_metrics/{}/housing_stock{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves housing event prices (median sale, list, rental prices).
    pub async fn housing_event_prices(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<MetricsResponse<HousingEventPrices>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/market_metrics/{}/housing_event_prices{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves all-cash transaction counts and percentages.
    pub async fn all_cash(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<MetricsResponse<AllCash>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/market_metrics/{}/all_cash{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves physical attributes of properties in housing events.
    pub async fn housing_event_property_attributes(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<MetricsResponse<HousingEventPropertyAttributes>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/market_metrics/{}/housing_event_property_attributes{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
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
    use crate::models::PropertyType;

    #[test]
    fn metrics_params_default() {
        let params = MetricsParams::new();
        assert!(params.limit.is_none());
        assert!(params.offset.is_none());
        assert!(params.start_date.is_none());
        assert!(params.end_date.is_none());
        assert!(params.property_type.is_none());
        assert!(!params.auto_paginate);
    }

    #[test]
    fn metrics_params_builder() {
        let params = MetricsParams::new()
            .limit(10)
            .offset(20)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .property_type(PropertyType::SingleFamily)
            .auto_paginate(true);

        assert_eq!(params.limit, Some(10));
        assert_eq!(params.offset, Some(20));
        assert_eq!(params.start_date, Some("2024-01-01".into()));
        assert_eq!(params.end_date, Some("2024-12-31".into()));
        assert_eq!(params.property_type, Some(PropertyType::SingleFamily));
        assert!(params.auto_paginate);
    }

    #[test]
    fn metrics_params_empty_query_string() {
        let params = MetricsParams::new();
        assert_eq!(params.to_query_string(), "");
    }

    #[test]
    fn metrics_params_query_string_limit() {
        let params = MetricsParams::new().limit(5);
        assert_eq!(params.to_query_string(), "?limit=5");
    }

    #[test]
    fn metrics_params_query_string_offset() {
        let params = MetricsParams::new().offset(10);
        assert_eq!(params.to_query_string(), "?offset=10");
    }

    #[test]
    fn metrics_params_query_string_dates() {
        let params = MetricsParams::new()
            .start_date("2024-01-01")
            .end_date("2024-06-30");

        let qs = params.to_query_string();
        assert!(qs.contains("start_date=2024-01-01"));
        assert!(qs.contains("end_date=2024-06-30"));
    }

    #[test]
    fn metrics_params_query_string_property_type() {
        let params = MetricsParams::new().property_type(PropertyType::Condo);
        assert_eq!(params.to_query_string(), "?property_type=CONDO");
    }

    #[test]
    fn metrics_params_query_string_all_fields() {
        let params = MetricsParams::new()
            .limit(10)
            .offset(5)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .property_type(PropertyType::Townhouse);

        let qs = params.to_query_string();
        assert!(qs.starts_with('?'));
        assert!(qs.contains("limit=10"));
        assert!(qs.contains("offset=5"));
        assert!(qs.contains("start_date=2024-01-01"));
        assert!(qs.contains("end_date=2024-12-31"));
        assert!(qs.contains("property_type=TOWNHOUSE"));
    }

    #[test]
    fn metrics_params_auto_paginate_not_in_query() {
        let params = MetricsParams::new().limit(5).auto_paginate(true);
        let qs = params.to_query_string();
        assert!(!qs.contains("auto_paginate"));
        assert!(qs.contains("limit=5"));
    }
}
