//! Market search endpoints for discovering Parcl market identifiers.

use crate::error::{ParclError, Result};
use crate::models::{LocationType, Market, PaginatedResponse};
use reqwest::Client;

/// Client for search API endpoints.
pub struct SearchClient<'a> {
    http: &'a Client,
    base_url: &'a str,
    api_key: &'a str,
}

impl<'a> SearchClient<'a> {
    pub(crate) fn new(http: &'a Client, base_url: &'a str, api_key: &'a str) -> Self {
        Self {
            http,
            base_url,
            api_key,
        }
    }

    /// Searches for markets by name, ZIP code, or other criteria.
    pub async fn markets(
        &self,
        query: &str,
        state: Option<&str>,
        location_type: Option<LocationType>,
        limit: Option<u32>,
    ) -> Result<PaginatedResponse<Market>> {
        let mut url = format!("{}/v1/search/markets", self.base_url);
        let mut params = vec![("query", query.to_string())];

        if let Some(s) = state {
            params.push(("state_abbreviation", s.to_string()));
        }
        if let Some(lt) = location_type {
            params.push(("location_type", lt.as_str().to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }

        let query_string: String = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        url = format!("{}?{}", url, query_string);

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

        let data: PaginatedResponse<Market> = response.json().await?;
        Ok(data)
    }

    /// Searches for all markets in a state.
    pub async fn markets_by_state(
        &self,
        state: &str,
        location_type: Option<LocationType>,
        limit: Option<u32>,
    ) -> Result<PaginatedResponse<Market>> {
        self.markets("", Some(state), location_type, limit).await
    }
}

mod urlencoding {
    pub fn encode(input: &str) -> String {
        let mut encoded = String::new();
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    encoded.push(byte as char);
                }
                b' ' => encoded.push_str("%20"),
                _ => encoded.push_str(&format!("%{:02X}", byte)),
            }
        }
        encoded
    }
}
