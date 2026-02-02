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

pub use endpoints::for_sale_metrics::ForSaleMetricsParams;
pub use endpoints::investor_metrics::InvestorMetricsParams;
pub use endpoints::market_metrics::MetricsParams;
pub use endpoints::new_construction_metrics::NewConstructionMetricsParams;
pub use endpoints::portfolio_metrics::PortfolioMetricsParams;
pub use endpoints::property::{EventHistoryParams, PropertySearchParams};
pub use endpoints::rental_metrics::RentalMetricsParams;
pub use endpoints::search::SearchParams;
pub use error::{ParclError, Result};
pub use models::*;
// RetryConfig is defined in this module (not models), so no re-export needed.

use endpoints::{
    ForSaleMetricsClient, InvestorMetricsClient, MarketMetricsClient, NewConstructionMetricsClient,
    PortfolioMetricsClient, PriceFeedClient, PropertyClient, RentalMetricsClient, SearchClient,
};
use reqwest::Client;
use std::env;
use std::sync::atomic::{AtomicI64, Ordering};

const DEFAULT_BASE_URL: &str = "https://api.parcllabs.com";
const ENV_API_KEY: &str = "PARCL_LABS_API_KEY";

/// Configuration for automatic retry on rate-limited (429) responses.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts before giving up.
    pub max_retries: u32,
    /// Initial backoff duration in milliseconds (doubles each attempt).
    pub initial_backoff_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_ms: 1000,
        }
    }
}

/// Main client for interacting with the Parcl Labs API.
pub struct ParclClient {
    pub(crate) http: Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    pub(crate) retry_config: RetryConfig,
    session_credits_used: AtomicI64,
    remaining_credits: AtomicI64,
}

impl std::fmt::Debug for ParclClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParclClient")
            .field("base_url", &self.base_url)
            .field("api_key", &"***")
            .field("retry_config", &self.retry_config)
            .field(
                "session_credits_used",
                &self.session_credits_used.load(Ordering::Relaxed),
            )
            .field(
                "remaining_credits",
                &self.remaining_credits.load(Ordering::Relaxed),
            )
            .finish()
    }
}

impl ParclClient {
    /// Creates a new client using the `PARCL_LABS_API_KEY` environment variable.
    pub fn new() -> Result<Self> {
        let api_key = env::var(ENV_API_KEY).map_err(|_| ParclError::MissingApiKey)?;
        Ok(Self {
            http: Client::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key,
            retry_config: RetryConfig::default(),
            session_credits_used: AtomicI64::new(0),
            remaining_credits: AtomicI64::new(0),
        })
    }

    /// Creates a new client with an explicit API key.
    pub fn with_api_key(api_key: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: api_key.into(),
            retry_config: RetryConfig::default(),
            session_credits_used: AtomicI64::new(0),
            remaining_credits: AtomicI64::new(0),
        }
    }

    /// Creates a new client with custom configuration.
    pub fn with_config(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
            retry_config: RetryConfig::default(),
            session_credits_used: AtomicI64::new(0),
            remaining_credits: AtomicI64::new(0),
        }
    }

    /// Sets the retry configuration for rate-limited requests.
    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Updates session credit tracking from an API response's account info.
    pub(crate) fn update_credits(&self, account: &Option<AccountInfo>) {
        if let Some(info) = account {
            if let Some(used) = info.est_credits_used {
                self.session_credits_used.fetch_add(used, Ordering::Relaxed);
            }
            if let Some(remaining) = info.est_remaining_credits {
                self.remaining_credits.store(remaining, Ordering::Relaxed);
            }
        }
    }

    /// Returns the accumulated session credit usage.
    pub fn account_info(&self) -> AccountUsage {
        AccountUsage {
            est_session_credits_used: self.session_credits_used.load(Ordering::Relaxed),
            est_remaining_credits: self.remaining_credits.load(Ordering::Relaxed),
        }
    }

    /// Returns total credits used in this session.
    pub fn session_credits_used(&self) -> i64 {
        self.session_credits_used.load(Ordering::Relaxed)
    }

    /// Returns the last known remaining credits.
    pub fn remaining_credits(&self) -> i64 {
        self.remaining_credits.load(Ordering::Relaxed)
    }

    /// Returns a client for search endpoints.
    pub fn search(&self) -> SearchClient<'_> {
        SearchClient::new(self)
    }

    /// Returns a client for market metrics endpoints.
    pub fn market_metrics(&self) -> MarketMetricsClient<'_> {
        MarketMetricsClient::new(self)
    }

    /// Returns a client for investor metrics endpoints.
    pub fn investor_metrics(&self) -> InvestorMetricsClient<'_> {
        InvestorMetricsClient::new(self)
    }

    /// Returns a client for for-sale market metrics endpoints.
    pub fn for_sale_metrics(&self) -> ForSaleMetricsClient<'_> {
        ForSaleMetricsClient::new(self)
    }

    /// Returns a client for rental market metrics endpoints.
    pub fn rental_metrics(&self) -> RentalMetricsClient<'_> {
        RentalMetricsClient::new(self)
    }

    /// Returns a client for price feed endpoints.
    pub fn price_feed(&self) -> PriceFeedClient<'_> {
        PriceFeedClient::new(self)
    }

    /// Returns a client for new construction metrics endpoints.
    pub fn new_construction_metrics(&self) -> NewConstructionMetricsClient<'_> {
        NewConstructionMetricsClient::new(self)
    }

    /// Returns a client for portfolio metrics endpoints.
    pub fn portfolio_metrics(&self) -> PortfolioMetricsClient<'_> {
        PortfolioMetricsClient::new(self)
    }

    /// Returns a client for property API endpoints.
    pub fn property(&self) -> PropertyClient<'_> {
        PropertyClient::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_with_api_key() {
        let client = ParclClient::with_api_key("test-key");
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url, DEFAULT_BASE_URL);
    }

    #[test]
    fn client_with_config() {
        let client = ParclClient::with_config("my-key", "https://custom.api.com");
        assert_eq!(client.api_key, "my-key");
        assert_eq!(client.base_url, "https://custom.api.com");
    }

    #[test]
    fn client_new_missing_env() {
        // Temporarily unset env var if it exists
        let original = env::var(ENV_API_KEY).ok();
        env::remove_var(ENV_API_KEY);

        let result = ParclClient::new();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParclError::MissingApiKey));

        // Restore original value
        if let Some(val) = original {
            env::set_var(ENV_API_KEY, val);
        }
    }

    #[test]
    fn client_new_with_env() {
        let original = env::var(ENV_API_KEY).ok();
        env::set_var(ENV_API_KEY, "env-test-key");

        let result = ParclClient::new();
        assert!(result.is_ok());
        let client = result.unwrap();
        assert_eq!(client.api_key, "env-test-key");

        // Restore original value
        if let Some(val) = original {
            env::set_var(ENV_API_KEY, val);
        } else {
            env::remove_var(ENV_API_KEY);
        }
    }

    #[test]
    fn client_with_retry_config() {
        let config = RetryConfig {
            max_retries: 5,
            initial_backoff_ms: 2000,
        };
        let client = ParclClient::with_api_key("test").with_retry_config(config);
        assert_eq!(client.retry_config.max_retries, 5);
        assert_eq!(client.retry_config.initial_backoff_ms, 2000);
    }

    #[test]
    fn retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_backoff_ms, 1000);
    }

    #[test]
    fn update_credits_accumulates() {
        let client = ParclClient::with_api_key("test");
        let info1 = Some(AccountInfo {
            est_credits_used: Some(10),
            est_remaining_credits: Some(990),
        });
        client.update_credits(&info1);
        assert_eq!(client.session_credits_used(), 10);
        assert_eq!(client.remaining_credits(), 990);

        let info2 = Some(AccountInfo {
            est_credits_used: Some(5),
            est_remaining_credits: Some(985),
        });
        client.update_credits(&info2);
        assert_eq!(client.session_credits_used(), 15);
        assert_eq!(client.remaining_credits(), 985);
    }

    #[test]
    fn update_credits_none_is_noop() {
        let client = ParclClient::with_api_key("test");
        client.update_credits(&None);
        assert_eq!(client.session_credits_used(), 0);
        assert_eq!(client.remaining_credits(), 0);
    }

    #[test]
    fn account_info_returns_session_state() {
        let client = ParclClient::with_api_key("test");
        let info = Some(AccountInfo {
            est_credits_used: Some(42),
            est_remaining_credits: Some(958),
        });
        client.update_credits(&info);
        let usage = client.account_info();
        assert_eq!(usage.est_session_credits_used, 42);
        assert_eq!(usage.est_remaining_credits, 958);
    }

    #[test]
    fn client_debug_hides_api_key() {
        let client = ParclClient::with_api_key("super-secret-key");
        let debug = format!("{:?}", client);
        assert!(!debug.contains("super-secret-key"));
        assert!(debug.contains("***"));
    }

    #[test]
    fn client_returns_search_client() {
        let client = ParclClient::with_api_key("test");
        let _search = client.search();
    }

    #[test]
    fn client_returns_market_metrics_client() {
        let client = ParclClient::with_api_key("test");
        let _metrics = client.market_metrics();
    }

    #[test]
    fn client_returns_investor_metrics_client() {
        let client = ParclClient::with_api_key("test");
        let _investor = client.investor_metrics();
    }

    #[test]
    fn client_returns_price_feed_client() {
        let client = ParclClient::with_api_key("test");
        let _feed = client.price_feed();
    }

    #[test]
    fn client_returns_for_sale_metrics_client() {
        let client = ParclClient::with_api_key("test");
        let _for_sale = client.for_sale_metrics();
    }

    #[test]
    fn client_returns_rental_metrics_client() {
        let client = ParclClient::with_api_key("test");
        let _rental = client.rental_metrics();
    }

    #[test]
    fn client_returns_new_construction_metrics_client() {
        let client = ParclClient::with_api_key("test");
        let _new_construction = client.new_construction_metrics();
    }

    #[test]
    fn client_returns_portfolio_metrics_client() {
        let client = ParclClient::with_api_key("test");
        let _portfolio = client.portfolio_metrics();
    }

    #[test]
    fn client_returns_property_client() {
        let client = ParclClient::with_api_key("test");
        let _property = client.property();
    }
}
