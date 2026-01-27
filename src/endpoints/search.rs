//! Market search endpoints for discovering Parcl market identifiers.

use crate::error::{ParclError, Result};
use crate::models::{LocationType, Market, PaginatedResponse, SortBy, SortOrder, USRegion};
use reqwest::Client;

/// Client for search API endpoints.
pub struct SearchClient<'a> {
    http: &'a Client,
    base_url: &'a str,
    api_key: &'a str,
}

/// Query parameters for market search.
#[derive(Debug, Default, Clone)]
pub struct SearchParams {
    pub query: Option<String>,
    pub location_type: Option<LocationType>,
    pub region: Option<USRegion>,
    pub state_abbreviation: Option<String>,
    pub state_fips_code: Option<String>,
    pub parcl_id: Option<i64>,
    pub geoid: Option<String>,
    pub sort_by: Option<SortBy>,
    pub sort_order: Option<SortOrder>,
    pub limit: Option<u32>,
    pub auto_paginate: bool,
}

impl SearchParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Search query (city name, ZIP code, etc.). Minimum 3 characters.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Filter by location type (City, County, Zip5, etc.)
    pub fn location_type(mut self, location_type: LocationType) -> Self {
        self.location_type = Some(location_type);
        self
    }

    /// Filter by US region (Pacific, Mountain, etc.)
    pub fn region(mut self, region: USRegion) -> Self {
        self.region = Some(region);
        self
    }

    /// Filter by state abbreviation (e.g., "CA", "NY")
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state_abbreviation = Some(state.into().to_uppercase());
        self
    }

    /// Filter by state FIPS code (e.g., "06" for California)
    pub fn state_fips_code(mut self, code: impl Into<String>) -> Self {
        self.state_fips_code = Some(code.into());
        self
    }

    /// Filter by specific parcl_id
    pub fn parcl_id(mut self, parcl_id: i64) -> Self {
        self.parcl_id = Some(parcl_id);
        self
    }

    /// Filter by geographic ID
    pub fn geoid(mut self, geoid: impl Into<String>) -> Self {
        self.geoid = Some(geoid.into());
        self
    }

    /// Sort results by field
    pub fn sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }

    /// Sort order (ascending or descending)
    pub fn sort_order(mut self, sort_order: SortOrder) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    /// Maximum number of results per page (default: API default)
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Automatically fetch all pages of results.
    pub fn auto_paginate(mut self, auto_paginate: bool) -> Self {
        self.auto_paginate = auto_paginate;
        self
    }

    pub(crate) fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(ref q) = self.query {
            params.push(format!("query={}", urlencoding::encode(q)));
        }
        if let Some(lt) = self.location_type {
            params.push(format!("location_type={}", lt.as_str()));
        }
        if let Some(r) = self.region {
            params.push(format!("region={}", r.as_str()));
        }
        if let Some(ref s) = self.state_abbreviation {
            params.push(format!("state_abbreviation={}", s));
        }
        if let Some(ref s) = self.state_fips_code {
            params.push(format!("state_fips_code={}", s));
        }
        if let Some(id) = self.parcl_id {
            params.push(format!("parcl_id={}", id));
        }
        if let Some(ref g) = self.geoid {
            params.push(format!("geoid={}", g));
        }
        if let Some(sb) = self.sort_by {
            params.push(format!("sort_by={}", sb.as_str()));
        }
        if let Some(so) = self.sort_order {
            params.push(format!("sort_order={}", so.as_str()));
        }
        if let Some(l) = self.limit {
            params.push(format!("limit={}", l));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

impl<'a> SearchClient<'a> {
    pub(crate) fn new(http: &'a Client, base_url: &'a str, api_key: &'a str) -> Self {
        Self {
            http,
            base_url,
            api_key,
        }
    }

    /// Searches for markets using the provided parameters.
    ///
    /// # Example
    /// ```no_run
    /// use parcllabs::{ParclClient, SearchParams, LocationType, SortBy, SortOrder};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ParclClient::new()?;
    ///
    /// // Single page
    /// let params = SearchParams::new()
    ///     .query("Los Angeles")
    ///     .state("CA")
    ///     .limit(10);
    /// let markets = client.search().markets(params).await?;
    ///
    /// // Auto-paginate to get all results
    /// let params = SearchParams::new()
    ///     .query("San")
    ///     .state("CA")
    ///     .auto_paginate(true);
    /// let all_markets = client.search().markets(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn markets(&self, params: SearchParams) -> Result<PaginatedResponse<Market>> {
        let query = params.to_query_string();
        let url = format!("{}/v1/search/markets{}", self.base_url, query);

        let mut response = self.fetch_page(&url).await?;

        if params.auto_paginate {
            while let Some(ref next_url) = response.links.next {
                let next_page = self.fetch_page(next_url).await?;
                response.items.extend(next_page.items);
                response.links = next_page.links;
            }
        }

        Ok(response)
    }

    async fn fetch_page(&self, url: &str) -> Result<PaginatedResponse<Market>> {
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

        let data: PaginatedResponse<Market> = response.json().await?;
        Ok(data)
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
