//! Market metrics endpoints for housing data retrieval.

use crate::error::{ParclError, Result};
use crate::models::{
    HousingEventCounts, HousingEventPrices, HousingStock, MetricsResponse, PropertyType,
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
}

impl MetricsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn start_date(mut self, date: impl Into<String>) -> Self {
        self.start_date = Some(date.into());
        self
    }

    pub fn end_date(mut self, date: impl Into<String>) -> Self {
        self.end_date = Some(date.into());
        self
    }

    /// Filter by property type (single family, condo, townhouse, etc.)
    pub fn property_type(mut self, property_type: PropertyType) -> Self {
        self.property_type = Some(property_type);
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
        let query = params.unwrap_or_default().to_query_string();
        let url = format!(
            "{}/v1/market_metrics/{}/housing_event_counts{}",
            self.base_url, parcl_id, query
        );
        self.fetch(&url).await
    }

    /// Retrieves housing stock data (single-family, condo, townhouse counts).
    pub async fn housing_stock(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<MetricsResponse<HousingStock>> {
        let query = params.unwrap_or_default().to_query_string();
        let url = format!(
            "{}/v1/market_metrics/{}/housing_stock{}",
            self.base_url, parcl_id, query
        );
        self.fetch(&url).await
    }

    /// Retrieves housing event prices (median sale, list, rental prices).
    pub async fn housing_event_prices(
        &self,
        parcl_id: i64,
        params: Option<MetricsParams>,
    ) -> Result<MetricsResponse<HousingEventPrices>> {
        let query = params.unwrap_or_default().to_query_string();
        let url = format!(
            "{}/v1/market_metrics/{}/housing_event_prices{}",
            self.base_url, parcl_id, query
        );
        self.fetch(&url).await
    }

    async fn fetch<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<MetricsResponse<T>> {
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
