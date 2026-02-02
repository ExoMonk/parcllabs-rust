//! For-sale market metrics endpoints for tracking inventory and listings.

use crate::error::{ParclError, Result};
use crate::models::{
    BatchMetricsResponse, ForSaleInventory, ForSaleInventoryPriceChanges, MetricsResponse,
    NewListingsRollingCounts, PropertyType,
};
use reqwest::Client;

/// Client for for-sale market metrics API endpoints.
pub struct ForSaleMetricsClient<'a> {
    http: &'a Client,
    base_url: &'a str,
    api_key: &'a str,
}

/// Query parameters for for-sale metrics requests.
#[derive(Debug, Default, Clone)]
pub struct ForSaleMetricsParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub property_type: Option<PropertyType>,
    pub auto_paginate: bool,
}

impl ForSaleMetricsParams {
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

    /// Filter by property type.
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
        if let Some(pt) = self.property_type {
            obj.insert("property_type".into(), serde_json::json!(pt.as_str()));
        }
        body
    }
}

impl<'a> ForSaleMetricsClient<'a> {
    pub(crate) fn new(http: &'a Client, base_url: &'a str, api_key: &'a str) -> Self {
        Self {
            http,
            base_url,
            api_key,
        }
    }

    /// Retrieves current for-sale inventory counts.
    ///
    /// Returns the total count of properties currently listed for sale in the market.
    /// Data series begins on September 1, 2022.
    pub async fn for_sale_inventory(
        &self,
        parcl_id: i64,
        params: Option<ForSaleMetricsParams>,
    ) -> Result<MetricsResponse<ForSaleInventory>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/for_sale_market_metrics/{}/for_sale_inventory{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves for-sale inventory price change metrics.
    ///
    /// Returns metrics on price behavior including price changes, price drops,
    /// median days between changes, and percentage of inventory affected.
    /// Data series begins on September 1, 2022.
    pub async fn for_sale_inventory_price_changes(
        &self,
        parcl_id: i64,
        params: Option<ForSaleMetricsParams>,
    ) -> Result<MetricsResponse<ForSaleInventoryPriceChanges>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/for_sale_market_metrics/{}/for_sale_inventory_price_changes{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves rolling counts of new for-sale listings.
    ///
    /// Returns rolling counts over 7, 30, 60, and 90 day periods
    /// for newly listed properties in the market.
    pub async fn new_listings_rolling_counts(
        &self,
        parcl_id: i64,
        params: Option<ForSaleMetricsParams>,
    ) -> Result<MetricsResponse<NewListingsRollingCounts>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/for_sale_market_metrics/{}/new_listings_rolling_counts{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    // --- Batch POST methods ---

    /// Batch retrieves for-sale inventory for multiple markets.
    pub async fn batch_for_sale_inventory(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<ForSaleMetricsParams>,
    ) -> Result<BatchMetricsResponse<ForSaleInventory>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/for_sale_market_metrics/for_sale_inventory", self.base_url);
        super::common::post_with_pagination(
            self.http,
            self.api_key,
            &url,
            &body,
            params.auto_paginate,
        )
        .await
    }

    /// Batch retrieves for-sale inventory price changes for multiple markets.
    pub async fn batch_for_sale_inventory_price_changes(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<ForSaleMetricsParams>,
    ) -> Result<BatchMetricsResponse<ForSaleInventoryPriceChanges>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/for_sale_market_metrics/for_sale_inventory_price_changes", self.base_url);
        super::common::post_with_pagination(
            self.http,
            self.api_key,
            &url,
            &body,
            params.auto_paginate,
        )
        .await
    }

    /// Batch retrieves new listings rolling counts for multiple markets.
    pub async fn batch_new_listings_rolling_counts(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<ForSaleMetricsParams>,
    ) -> Result<BatchMetricsResponse<NewListingsRollingCounts>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/for_sale_market_metrics/new_listings_rolling_counts", self.base_url);
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

    #[test]
    fn for_sale_params_default() {
        let params = ForSaleMetricsParams::new();
        assert!(params.limit.is_none());
        assert!(params.offset.is_none());
        assert!(params.start_date.is_none());
        assert!(params.end_date.is_none());
        assert!(params.property_type.is_none());
        assert!(!params.auto_paginate);
    }

    #[test]
    fn for_sale_params_builder() {
        let params = ForSaleMetricsParams::new()
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
    fn for_sale_params_empty_query_string() {
        let params = ForSaleMetricsParams::new();
        assert_eq!(params.to_query_string(), "");
    }

    #[test]
    fn for_sale_params_query_string_all_fields() {
        let params = ForSaleMetricsParams::new()
            .limit(10)
            .offset(5)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .property_type(PropertyType::Condo);

        let qs = params.to_query_string();
        assert!(qs.starts_with('?'));
        assert!(qs.contains("limit=10"));
        assert!(qs.contains("offset=5"));
        assert!(qs.contains("start_date=2024-01-01"));
        assert!(qs.contains("end_date=2024-12-31"));
        assert!(qs.contains("property_type=CONDO"));
    }

    #[test]
    fn for_sale_params_query_string_partial() {
        let params = ForSaleMetricsParams::new()
            .limit(25)
            .property_type(PropertyType::Townhouse);

        let qs = params.to_query_string();
        assert!(qs.starts_with('?'));
        assert!(qs.contains("limit=25"));
        assert!(qs.contains("property_type=TOWNHOUSE"));
        assert!(!qs.contains("offset"));
        assert!(!qs.contains("start_date"));
    }

    #[test]
    fn for_sale_params_batch_body_minimal() {
        let params = ForSaleMetricsParams::new();
        let body = params.to_batch_body(&[100, 200]);
        let obj = body.as_object().unwrap();
        assert_eq!(obj["parcl_id"], serde_json::json!([100, 200]));
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn for_sale_params_batch_body_all_fields() {
        let params = ForSaleMetricsParams::new()
            .limit(10)
            .offset(5)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .property_type(PropertyType::Condo);
        let body = params.to_batch_body(&[100]);
        let obj = body.as_object().unwrap();
        assert_eq!(obj["parcl_id"], serde_json::json!([100]));
        assert_eq!(obj["limit"], 10);
        assert_eq!(obj["offset"], 5);
        assert_eq!(obj["start_date"], "2024-01-01");
        assert_eq!(obj["end_date"], "2024-12-31");
        assert_eq!(obj["property_type"], "CONDO");
    }
}
