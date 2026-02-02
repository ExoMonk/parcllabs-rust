//! Property API endpoints for searching properties and retrieving event history.

use crate::error::{ParclError, Result};
use crate::models::{
    AddressSearchRequest, EntityOwnerName, EventType, PropertyEventHistoryResponse, PropertyType,
    PropertySearchResponse, PropertyV2SearchRequest, PropertyV2SearchResponse,
};
use crate::ParclClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Client for property API endpoints.
pub struct PropertyClient<'a> {
    client: &'a ParclClient,
}

/// Query parameters for the `GET /v1/property/search` endpoint.
#[derive(Debug, Clone)]
pub struct PropertySearchParams {
    pub parcl_id: i64,
    pub property_type: PropertyType,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub square_footage_min: Option<i64>,
    pub square_footage_max: Option<i64>,
    pub bedrooms_min: Option<i32>,
    pub bedrooms_max: Option<i32>,
    pub bathrooms_min: Option<i32>,
    pub bathrooms_max: Option<i32>,
    pub year_built_min: Option<i32>,
    pub year_built_max: Option<i32>,
    pub current_entity_owner_name: Option<EntityOwnerName>,
    pub event_history_sale_flag: Option<bool>,
    pub event_history_rental_flag: Option<bool>,
    pub event_history_listing_flag: Option<bool>,
    pub current_new_construction_flag: Option<bool>,
    pub current_owner_occupied_flag: Option<bool>,
    pub current_investor_owned_flag: Option<bool>,
    pub current_on_market_flag: Option<bool>,
    pub current_on_market_rental_flag: Option<bool>,
    pub record_added_date_start: Option<String>,
    pub record_added_date_end: Option<String>,
}

impl PropertySearchParams {
    /// Create params with required fields.
    pub fn new(parcl_id: i64, property_type: PropertyType) -> Self {
        Self {
            parcl_id,
            property_type,
            limit: None,
            offset: None,
            square_footage_min: None,
            square_footage_max: None,
            bedrooms_min: None,
            bedrooms_max: None,
            bathrooms_min: None,
            bathrooms_max: None,
            year_built_min: None,
            year_built_max: None,
            current_entity_owner_name: None,
            event_history_sale_flag: None,
            event_history_rental_flag: None,
            event_history_listing_flag: None,
            current_new_construction_flag: None,
            current_owner_occupied_flag: None,
            current_investor_owned_flag: None,
            current_on_market_flag: None,
            current_on_market_rental_flag: None,
            record_added_date_start: None,
            record_added_date_end: None,
        }
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

    pub fn square_footage_min(mut self, v: i64) -> Self {
        self.square_footage_min = Some(v);
        self
    }

    pub fn square_footage_max(mut self, v: i64) -> Self {
        self.square_footage_max = Some(v);
        self
    }

    pub fn bedrooms_min(mut self, v: i32) -> Self {
        self.bedrooms_min = Some(v);
        self
    }

    pub fn bedrooms_max(mut self, v: i32) -> Self {
        self.bedrooms_max = Some(v);
        self
    }

    pub fn bathrooms_min(mut self, v: i32) -> Self {
        self.bathrooms_min = Some(v);
        self
    }

    pub fn bathrooms_max(mut self, v: i32) -> Self {
        self.bathrooms_max = Some(v);
        self
    }

    pub fn year_built_min(mut self, v: i32) -> Self {
        self.year_built_min = Some(v);
        self
    }

    pub fn year_built_max(mut self, v: i32) -> Self {
        self.year_built_max = Some(v);
        self
    }

    pub fn current_entity_owner_name(mut self, name: EntityOwnerName) -> Self {
        self.current_entity_owner_name = Some(name);
        self
    }

    pub fn event_history_sale_flag(mut self, v: bool) -> Self {
        self.event_history_sale_flag = Some(v);
        self
    }

    pub fn event_history_rental_flag(mut self, v: bool) -> Self {
        self.event_history_rental_flag = Some(v);
        self
    }

    pub fn event_history_listing_flag(mut self, v: bool) -> Self {
        self.event_history_listing_flag = Some(v);
        self
    }

    pub fn current_new_construction_flag(mut self, v: bool) -> Self {
        self.current_new_construction_flag = Some(v);
        self
    }

    pub fn current_owner_occupied_flag(mut self, v: bool) -> Self {
        self.current_owner_occupied_flag = Some(v);
        self
    }

    pub fn current_investor_owned_flag(mut self, v: bool) -> Self {
        self.current_investor_owned_flag = Some(v);
        self
    }

    pub fn current_on_market_flag(mut self, v: bool) -> Self {
        self.current_on_market_flag = Some(v);
        self
    }

    pub fn current_on_market_rental_flag(mut self, v: bool) -> Self {
        self.current_on_market_rental_flag = Some(v);
        self
    }

    pub fn record_added_date_start(mut self, date: impl Into<String>) -> Self {
        self.record_added_date_start = Some(date.into());
        self
    }

    pub fn record_added_date_end(mut self, date: impl Into<String>) -> Self {
        self.record_added_date_end = Some(date.into());
        self
    }

    pub(crate) fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        params.push(format!("parcl_id={}", self.parcl_id));
        params.push(format!("property_type={}", self.property_type.as_str()));

        if let Some(l) = self.limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = self.offset {
            params.push(format!("offset={}", o));
        }
        if let Some(v) = self.square_footage_min {
            params.push(format!("square_footage_min={}", v));
        }
        if let Some(v) = self.square_footage_max {
            params.push(format!("square_footage_max={}", v));
        }
        if let Some(v) = self.bedrooms_min {
            params.push(format!("bedrooms_min={}", v));
        }
        if let Some(v) = self.bedrooms_max {
            params.push(format!("bedrooms_max={}", v));
        }
        if let Some(v) = self.bathrooms_min {
            params.push(format!("bathrooms_min={}", v));
        }
        if let Some(v) = self.bathrooms_max {
            params.push(format!("bathrooms_max={}", v));
        }
        if let Some(v) = self.year_built_min {
            params.push(format!("year_built_min={}", v));
        }
        if let Some(v) = self.year_built_max {
            params.push(format!("year_built_max={}", v));
        }
        if let Some(ref name) = self.current_entity_owner_name {
            params.push(format!("current_entity_owner_name={}", name.as_str()));
        }
        if let Some(v) = self.event_history_sale_flag {
            params.push(format!("event_history_sale_flag={}", v as i32));
        }
        if let Some(v) = self.event_history_rental_flag {
            params.push(format!("event_history_rental_flag={}", v as i32));
        }
        if let Some(v) = self.event_history_listing_flag {
            params.push(format!("event_history_listing_flag={}", v as i32));
        }
        if let Some(v) = self.current_new_construction_flag {
            params.push(format!("current_new_construction_flag={}", v as i32));
        }
        if let Some(v) = self.current_owner_occupied_flag {
            params.push(format!("current_owner_occupied_flag={}", v as i32));
        }
        if let Some(v) = self.current_investor_owned_flag {
            params.push(format!("current_investor_owned_flag={}", v as i32));
        }
        if let Some(v) = self.current_on_market_flag {
            params.push(format!("current_on_market_flag={}", v as i32));
        }
        if let Some(v) = self.current_on_market_rental_flag {
            params.push(format!("current_on_market_rental_flag={}", v as i32));
        }
        if let Some(ref d) = self.record_added_date_start {
            params.push(format!("record_added_date_start={}", d));
        }
        if let Some(ref d) = self.record_added_date_end {
            params.push(format!("record_added_date_end={}", d));
        }

        format!("?{}", params.join("&"))
    }
}

/// Builder for `POST /v1/property/event_history` request parameters.
#[derive(Debug, Clone)]
pub struct EventHistoryParams {
    pub parcl_property_ids: Vec<i64>,
    pub event_type: Option<EventType>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub entity_owner_name: Option<EntityOwnerName>,
    pub record_updated_date_start: Option<String>,
    pub record_updated_date_end: Option<String>,
}

impl EventHistoryParams {
    /// Create params with required property IDs (max 1000).
    pub fn new(parcl_property_ids: Vec<i64>) -> Self {
        Self {
            parcl_property_ids,
            event_type: None,
            start_date: None,
            end_date: None,
            entity_owner_name: None,
            record_updated_date_start: None,
            record_updated_date_end: None,
        }
    }

    pub fn event_type(mut self, event_type: EventType) -> Self {
        self.event_type = Some(event_type);
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

    pub fn entity_owner_name(mut self, name: EntityOwnerName) -> Self {
        self.entity_owner_name = Some(name);
        self
    }

    pub fn record_updated_date_start(mut self, date: impl Into<String>) -> Self {
        self.record_updated_date_start = Some(date.into());
        self
    }

    pub fn record_updated_date_end(mut self, date: impl Into<String>) -> Self {
        self.record_updated_date_end = Some(date.into());
        self
    }

    fn to_request_body(&self) -> serde_json::Value {
        let mut body = serde_json::json!({
            "parcl_property_id": self.parcl_property_ids,
        });
        let obj = body.as_object_mut().unwrap();

        if let Some(ref et) = self.event_type {
            obj.insert("event_type".into(), serde_json::json!(et.as_str()));
        }
        if let Some(ref d) = self.start_date {
            obj.insert("start_date".into(), serde_json::json!(d));
        }
        if let Some(ref d) = self.end_date {
            obj.insert("end_date".into(), serde_json::json!(d));
        }
        if let Some(ref name) = self.entity_owner_name {
            obj.insert("entity_owner_name".into(), serde_json::json!(name.as_str()));
        }
        if let Some(ref d) = self.record_updated_date_start {
            obj.insert("record_updated_date_start".into(), serde_json::json!(d));
        }
        if let Some(ref d) = self.record_updated_date_end {
            obj.insert("record_updated_date_end".into(), serde_json::json!(d));
        }

        body
    }
}

impl<'a> PropertyClient<'a> {
    pub(crate) fn new(client: &'a ParclClient) -> Self {
        Self { client }
    }

    /// Search properties in a market by filters.
    ///
    /// `GET /v1/property/search`
    pub async fn search(
        &self,
        params: PropertySearchParams,
    ) -> Result<PropertySearchResponse> {
        let query = params.to_query_string();
        let url = format!("{}/v1/property/search{}", self.client.base_url, query);
        let resp: PropertySearchResponse = self.fetch_get(&url).await?;
        self.client.update_credits(&resp.account);
        Ok(resp)
    }

    /// Look up property IDs by street address.
    ///
    /// `POST /v1/property/search_address`
    pub async fn search_by_address(
        &self,
        addresses: Vec<AddressSearchRequest>,
    ) -> Result<PropertySearchResponse> {
        let url = format!("{}/v1/property/search_address", self.client.base_url);
        let resp: PropertySearchResponse = self.fetch_post(&url, &addresses).await?;
        self.client.update_credits(&resp.account);
        Ok(resp)
    }

    /// Get event history for a list of property IDs.
    ///
    /// `POST /v1/property/event_history`
    pub async fn event_history(
        &self,
        params: EventHistoryParams,
    ) -> Result<PropertyEventHistoryResponse> {
        let url = format!("{}/v1/property/event_history", self.client.base_url);
        let body = params.to_request_body();
        self.fetch_post(&url, &body).await
    }

    /// Advanced property search with nested filters (v2).
    ///
    /// `POST /v2/property_search`
    pub async fn search_v2(
        &self,
        request: PropertyV2SearchRequest,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<PropertyV2SearchResponse> {
        let mut query_parts = Vec::new();
        if let Some(l) = limit {
            query_parts.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            query_parts.push(format!("offset={}", o));
        }
        let query = if query_parts.is_empty() {
            String::new()
        } else {
            format!("?{}", query_parts.join("&"))
        };
        let url = format!("{}/v2/property_search{}", self.client.base_url, query);
        self.fetch_post(&url, &request).await
    }

    async fn fetch_get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        for attempt in 0..=self.client.retry_config.max_retries {
            let response = self
                .client
                .http
                .get(url)
                .header("Authorization", &self.client.api_key)
                .send()
                .await?;

            let status = response.status();
            if status.as_u16() == 429 && attempt < self.client.retry_config.max_retries {
                let backoff =
                    self.client.retry_config.initial_backoff_ms * 2u64.pow(attempt);
                tokio::time::sleep(std::time::Duration::from_millis(backoff)).await;
                continue;
            }

            if !status.is_success() {
                let message = response.text().await.unwrap_or_default();
                if status.as_u16() == 429 {
                    return Err(ParclError::RateLimited {
                        attempts: attempt + 1,
                        message,
                    });
                }
                return Err(ParclError::ApiError {
                    status: status.as_u16(),
                    message,
                });
            }

            let data: T = response.json().await?;
            return Ok(data);
        }
        unreachable!()
    }

    async fn fetch_post<B: Serialize, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T> {
        for attempt in 0..=self.client.retry_config.max_retries {
            let response = self
                .client
                .http
                .post(url)
                .header("Authorization", &self.client.api_key)
                .json(body)
                .send()
                .await?;

            let status = response.status();
            if status.as_u16() == 429 && attempt < self.client.retry_config.max_retries {
                let backoff =
                    self.client.retry_config.initial_backoff_ms * 2u64.pow(attempt);
                tokio::time::sleep(std::time::Duration::from_millis(backoff)).await;
                continue;
            }

            if !status.is_success() {
                let message = response.text().await.unwrap_or_default();
                if status.as_u16() == 429 {
                    return Err(ParclError::RateLimited {
                        attempts: attempt + 1,
                        message,
                    });
                }
                return Err(ParclError::ApiError {
                    status: status.as_u16(),
                    message,
                });
            }

            let data: T = response.json().await?;
            return Ok(data);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{EntityOwnerName, EventType, PropertyType};

    #[test]
    fn property_search_params_required_only() {
        let params = PropertySearchParams::new(5387853, PropertyType::SingleFamily);
        let qs = params.to_query_string();
        assert!(qs.starts_with('?'));
        assert!(qs.contains("parcl_id=5387853"));
        assert!(qs.contains("property_type=SINGLE_FAMILY"));
    }

    #[test]
    fn property_search_params_with_filters() {
        let params = PropertySearchParams::new(5387853, PropertyType::Condo)
            .limit(10)
            .offset(20)
            .bedrooms_min(2)
            .bedrooms_max(4)
            .bathrooms_min(1)
            .square_footage_min(1000)
            .year_built_min(2000)
            .current_entity_owner_name(EntityOwnerName::InvitationHomes)
            .event_history_sale_flag(true)
            .current_on_market_flag(false);

        let qs = params.to_query_string();
        assert!(qs.contains("limit=10"));
        assert!(qs.contains("offset=20"));
        assert!(qs.contains("bedrooms_min=2"));
        assert!(qs.contains("bedrooms_max=4"));
        assert!(qs.contains("bathrooms_min=1"));
        assert!(qs.contains("square_footage_min=1000"));
        assert!(qs.contains("year_built_min=2000"));
        assert!(qs.contains("current_entity_owner_name=INVITATION_HOMES"));
        assert!(qs.contains("event_history_sale_flag=1"));
        assert!(qs.contains("current_on_market_flag=0"));
    }

    #[test]
    fn property_search_params_bool_flags_as_int() {
        let params = PropertySearchParams::new(123, PropertyType::AllProperties)
            .event_history_sale_flag(true)
            .current_investor_owned_flag(false);

        let qs = params.to_query_string();
        assert!(qs.contains("event_history_sale_flag=1"));
        assert!(qs.contains("current_investor_owned_flag=0"));
    }

    #[test]
    fn property_search_params_date_filters() {
        let params = PropertySearchParams::new(123, PropertyType::SingleFamily)
            .record_added_date_start("2024-01-01")
            .record_added_date_end("2024-12-31");

        let qs = params.to_query_string();
        assert!(qs.contains("record_added_date_start=2024-01-01"));
        assert!(qs.contains("record_added_date_end=2024-12-31"));
    }

    #[test]
    fn event_history_params_required_only() {
        let params = EventHistoryParams::new(vec![123, 456]);
        let body = params.to_request_body();
        assert_eq!(body["parcl_property_id"], serde_json::json!([123, 456]));
        assert!(body.get("event_type").is_none());
    }

    #[test]
    fn event_history_params_with_filters() {
        let params = EventHistoryParams::new(vec![123])
            .event_type(EventType::Sale)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .entity_owner_name(EntityOwnerName::Amh);

        let body = params.to_request_body();
        assert_eq!(body["parcl_property_id"], serde_json::json!([123]));
        assert_eq!(body["event_type"], "SALE");
        assert_eq!(body["start_date"], "2024-01-01");
        assert_eq!(body["end_date"], "2024-12-31");
        assert_eq!(body["entity_owner_name"], "AMH");
    }

    #[test]
    fn event_history_params_record_updated_dates() {
        let params = EventHistoryParams::new(vec![1])
            .record_updated_date_start("2024-06-01")
            .record_updated_date_end("2024-12-31");

        let body = params.to_request_body();
        assert_eq!(body["record_updated_date_start"], "2024-06-01");
        assert_eq!(body["record_updated_date_end"], "2024-12-31");
    }
}
