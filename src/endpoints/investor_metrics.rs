//! Investor metrics endpoints for tracking institutional activity.

use crate::error::Result;
use crate::models::{
    BatchMetricsResponse, HousingEventPrices, InvestorHousingEventCounts,
    InvestorHousingStockOwnership, InvestorNewListingsRollingCounts,
    InvestorPurchaseToSaleRatio, MetricsResponse, PropertyType,
};
use crate::ParclClient;

/// Client for investor metrics API endpoints.
pub struct InvestorMetricsClient<'a> {
    client: &'a ParclClient,
}

/// Query parameters for investor metrics requests.
#[derive(Debug, Default, Clone)]
pub struct InvestorMetricsParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub property_type: Option<PropertyType>,
    pub auto_paginate: bool,
}

impl InvestorMetricsParams {
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

impl<'a> InvestorMetricsClient<'a> {
    pub(crate) fn new(client: &'a ParclClient) -> Self {
        Self { client }
    }

    /// Retrieves investor housing stock ownership data.
    pub async fn housing_stock_ownership(
        &self,
        parcl_id: i64,
        params: Option<InvestorMetricsParams>,
    ) -> Result<MetricsResponse<InvestorHousingStockOwnership>> {
        let params = params.unwrap_or_default();
        let url = format!(
            "{}/v1/investor_metrics/{}/housing_stock_ownership{}",
            self.client.base_url, parcl_id, params.to_query_string()
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

    /// Retrieves investor purchase-to-sale ratio.
    pub async fn purchase_to_sale_ratio(
        &self,
        parcl_id: i64,
        params: Option<InvestorMetricsParams>,
    ) -> Result<MetricsResponse<InvestorPurchaseToSaleRatio>> {
        let params = params.unwrap_or_default();
        let url = format!(
            "{}/v1/investor_metrics/{}/purchase_to_sale_ratio{}",
            self.client.base_url, parcl_id, params.to_query_string()
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

    /// Retrieves investor housing event counts.
    pub async fn housing_event_counts(
        &self,
        parcl_id: i64,
        params: Option<InvestorMetricsParams>,
    ) -> Result<MetricsResponse<InvestorHousingEventCounts>> {
        let params = params.unwrap_or_default();
        let url = format!(
            "{}/v1/investor_metrics/{}/housing_event_counts{}",
            self.client.base_url, parcl_id, params.to_query_string()
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

    /// Retrieves investor housing event prices.
    pub async fn housing_event_prices(
        &self,
        parcl_id: i64,
        params: Option<InvestorMetricsParams>,
    ) -> Result<MetricsResponse<HousingEventPrices>> {
        let params = params.unwrap_or_default();
        let url = format!(
            "{}/v1/investor_metrics/{}/housing_event_prices{}",
            self.client.base_url, parcl_id, params.to_query_string()
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

    /// Retrieves rolling counts of investor new listings for sale.
    pub async fn new_listings_for_sale_rolling_counts(
        &self,
        parcl_id: i64,
        params: Option<InvestorMetricsParams>,
    ) -> Result<MetricsResponse<InvestorNewListingsRollingCounts>> {
        let params = params.unwrap_or_default();
        let url = format!(
            "{}/v1/investor_metrics/{}/new_listings_for_sale_rolling_counts{}",
            self.client.base_url, parcl_id, params.to_query_string()
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

    /// Batch retrieves housing stock ownership for multiple markets.
    pub async fn batch_housing_stock_ownership(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<InvestorMetricsParams>,
    ) -> Result<BatchMetricsResponse<InvestorHousingStockOwnership>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/investor_metrics/housing_stock_ownership", self.client.base_url);
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

    /// Batch retrieves purchase-to-sale ratio for multiple markets.
    pub async fn batch_purchase_to_sale_ratio(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<InvestorMetricsParams>,
    ) -> Result<BatchMetricsResponse<InvestorPurchaseToSaleRatio>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/investor_metrics/purchase_to_sale_ratio", self.client.base_url);
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

    /// Batch retrieves housing event counts for multiple markets.
    pub async fn batch_housing_event_counts(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<InvestorMetricsParams>,
    ) -> Result<BatchMetricsResponse<InvestorHousingEventCounts>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/investor_metrics/housing_event_counts", self.client.base_url);
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

    /// Batch retrieves housing event prices for multiple markets.
    pub async fn batch_housing_event_prices(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<InvestorMetricsParams>,
    ) -> Result<BatchMetricsResponse<HousingEventPrices>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/investor_metrics/housing_event_prices", self.client.base_url);
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

    /// Batch retrieves new listings for sale rolling counts for multiple markets.
    pub async fn batch_new_listings_for_sale_rolling_counts(
        &self,
        parcl_ids: Vec<i64>,
        params: Option<InvestorMetricsParams>,
    ) -> Result<BatchMetricsResponse<InvestorNewListingsRollingCounts>> {
        let params = params.unwrap_or_default();
        let body = params.to_batch_body(&parcl_ids);
        let url = format!("{}/v1/investor_metrics/new_listings_for_sale_rolling_counts", self.client.base_url);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn investor_params_default() {
        let params = InvestorMetricsParams::new();
        assert!(params.limit.is_none());
        assert!(params.offset.is_none());
        assert!(params.start_date.is_none());
        assert!(params.end_date.is_none());
        assert!(params.property_type.is_none());
        assert!(!params.auto_paginate);
    }

    #[test]
    fn investor_params_builder() {
        let params = InvestorMetricsParams::new()
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
    fn investor_params_empty_query_string() {
        let params = InvestorMetricsParams::new();
        assert_eq!(params.to_query_string(), "");
    }

    #[test]
    fn investor_params_query_string_all_fields() {
        let params = InvestorMetricsParams::new()
            .limit(10)
            .offset(5)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .property_type(PropertyType::SingleFamily);

        let qs = params.to_query_string();
        assert!(qs.starts_with('?'));
        assert!(qs.contains("limit=10"));
        assert!(qs.contains("offset=5"));
        assert!(qs.contains("start_date=2024-01-01"));
        assert!(qs.contains("end_date=2024-12-31"));
        assert!(qs.contains("property_type=SINGLE_FAMILY"));
    }

    #[test]
    fn investor_params_batch_body_minimal() {
        let params = InvestorMetricsParams::new();
        let body = params.to_batch_body(&[100, 200]);
        let obj = body.as_object().unwrap();
        assert_eq!(obj["parcl_id"], serde_json::json!([100, 200]));
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn investor_params_batch_body_all_fields() {
        let params = InvestorMetricsParams::new()
            .limit(10)
            .offset(5)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .property_type(PropertyType::SingleFamily);
        let body = params.to_batch_body(&[100]);
        let obj = body.as_object().unwrap();
        assert_eq!(obj["parcl_id"], serde_json::json!([100]));
        assert_eq!(obj["limit"], 10);
        assert_eq!(obj["offset"], 5);
        assert_eq!(obj["start_date"], "2024-01-01");
        assert_eq!(obj["end_date"], "2024-12-31");
        assert_eq!(obj["property_type"], "SINGLE_FAMILY");
    }
}
