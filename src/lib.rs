//! Rust SDK for the Parcl Labs API.
//!
//! Provides async access to U.S. housing market data including market search,
//! housing metrics, and price feeds.
//!
//! # Example
//!
//! ```no_run
//! use parcllabs::{ParclClient, SearchParams};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ParclClient::new()?;
//!
//!     // Search for markets
//!     let params = SearchParams::new().query("Los Angeles").limit(5);
//!     let markets = client.search().markets(params).await?;
//!     let la_market = &markets.items[0];
//!     println!("Found: {} (parcl_id: {})", la_market.name, la_market.parcl_id);
//!
//!     // Get housing metrics
//!     let events = client
//!         .market_metrics()
//!         .housing_event_counts(la_market.parcl_id, None)
//!         .await?;
//!
//!     for event in events.items.iter().take(3) {
//!         println!("{}: {} sales", event.date, event.sales.unwrap_or(0));
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod endpoints;
pub mod error;
pub mod models;

pub use endpoints::market_metrics::MetricsParams;
pub use endpoints::search::SearchParams;
pub use error::{ParclError, Result};
pub use models::*;

use endpoints::{MarketMetricsClient, PriceFeedClient, SearchClient};
use reqwest::Client;
use std::env;

const DEFAULT_BASE_URL: &str = "https://api.parcllabs.com";
const ENV_API_KEY: &str = "PARCL_LABS_API_KEY";

/// Main client for interacting with the Parcl Labs API.
pub struct ParclClient {
    http: Client,
    base_url: String,
    api_key: String,
}

impl ParclClient {
    /// Creates a new client using the `PARCL_LABS_API_KEY` environment variable.
    pub fn new() -> Result<Self> {
        let api_key = env::var(ENV_API_KEY).map_err(|_| ParclError::MissingApiKey)?;
        Ok(Self {
            http: Client::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key,
        })
    }

    /// Creates a new client with an explicit API key.
    pub fn with_api_key(api_key: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: api_key.into(),
        }
    }

    /// Creates a new client with custom configuration.
    pub fn with_config(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }

    /// Returns a client for search endpoints.
    pub fn search(&self) -> SearchClient<'_> {
        SearchClient::new(&self.http, &self.base_url, &self.api_key)
    }

    /// Returns a client for market metrics endpoints.
    pub fn market_metrics(&self) -> MarketMetricsClient<'_> {
        MarketMetricsClient::new(&self.http, &self.base_url, &self.api_key)
    }

    /// Returns a client for price feed endpoints.
    pub fn price_feed(&self) -> PriceFeedClient<'_> {
        PriceFeedClient::new(&self.http, &self.base_url, &self.api_key)
    }
}
