//! Investor metrics endpoints for tracking institutional activity.

use crate::error::{ParclError, Result};
use crate::models::{
    HousingEventPrices, InvestorHousingEventCounts, InvestorHousingStockOwnership,
    InvestorNewListingsRollingCounts, InvestorPurchaseToSaleRatio, MetricsResponse, PropertyType,
};
use reqwest::Client;

/// Client for investor metrics API endpoints.
pub struct InvestorMetricsClient<'a> {
    http: &'a Client,
    base_url: &'a str,
    api_key: &'a str,
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
}

impl<'a> InvestorMetricsClient<'a> {
    pub(crate) fn new(http: &'a Client, base_url: &'a str, api_key: &'a str) -> Self {
        Self {
            http,
            base_url,
            api_key,
        }
    }

    /// Retrieves investor housing stock ownership data.
    pub async fn housing_stock_ownership(
        &self,
        parcl_id: i64,
        params: Option<InvestorMetricsParams>,
    ) -> Result<MetricsResponse<InvestorHousingStockOwnership>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/investor_metrics/{}/housing_stock_ownership{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves investor purchase-to-sale ratio.
    pub async fn purchase_to_sale_ratio(
        &self,
        parcl_id: i64,
        params: Option<InvestorMetricsParams>,
    ) -> Result<MetricsResponse<InvestorPurchaseToSaleRatio>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/investor_metrics/{}/purchase_to_sale_ratio{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves investor housing event counts.
    pub async fn housing_event_counts(
        &self,
        parcl_id: i64,
        params: Option<InvestorMetricsParams>,
    ) -> Result<MetricsResponse<InvestorHousingEventCounts>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/investor_metrics/{}/housing_event_counts{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves investor housing event prices.
    pub async fn housing_event_prices(
        &self,
        parcl_id: i64,
        params: Option<InvestorMetricsParams>,
    ) -> Result<MetricsResponse<HousingEventPrices>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/investor_metrics/{}/housing_event_prices{}",
            self.base_url, parcl_id, query
        );
        self.fetch_with_pagination(&url, auto_paginate).await
    }

    /// Retrieves rolling counts of investor new listings for sale.
    pub async fn new_listings_for_sale_rolling_counts(
        &self,
        parcl_id: i64,
        params: Option<InvestorMetricsParams>,
    ) -> Result<MetricsResponse<InvestorNewListingsRollingCounts>> {
        let params = params.unwrap_or_default();
        let auto_paginate = params.auto_paginate;
        let query = params.to_query_string();
        let url = format!(
            "{}/v1/investor_metrics/{}/new_listings_for_sale_rolling_counts{}",
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
}
