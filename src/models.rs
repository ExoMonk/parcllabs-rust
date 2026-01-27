//! Response types for the Parcl Labs API.

use serde::{Deserialize, Serialize};

/// Paginated API response wrapper (for search endpoints).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub limit: u64,
    pub offset: u64,
    pub links: PaginationLinks,
}

/// Paginated response for market metrics (includes parcl_id at top level).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetricsResponse<T> {
    pub parcl_id: i64,
    pub items: Vec<T>,
    pub total: u64,
    pub limit: u64,
    pub offset: u64,
    pub links: PaginationLinks,
}

/// Navigation links for paginated responses.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationLinks {
    pub first: Option<String>,
    pub next: Option<String>,
    pub prev: Option<String>,
    pub last: Option<String>,
}

// ============================================================================
// Search
// ============================================================================

/// A housing market returned from search.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Market {
    /// Unique Parcl market identifier.
    pub parcl_id: i64,
    pub name: String,
    pub state_abbreviation: Option<String>,
    pub state_fips_code: Option<String>,
    pub location_type: String,
    pub total_population: Option<i64>,
    pub median_income: Option<i64>,
    /// Whether this market is tradeable on the Parcl exchange.
    pub parcl_exchange_market: Option<i32>,
    /// Whether this market has price feed data.
    pub pricefeed_market: Option<i32>,
}

/// Location type filter for market search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationType {
    City,
    County,
    Zip,
    State,
    Metro,
    Region,
    CensusPlace,
    National,
}

impl LocationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::City => "CITY",
            Self::County => "COUNTY",
            Self::Zip => "ZIP",
            Self::State => "STATE",
            Self::Metro => "METRO",
            Self::Region => "REGION",
            Self::CensusPlace => "CENSUS_PLACE",
            Self::National => "NATIONAL",
        }
    }
}

// ============================================================================
// Market Metrics
// ============================================================================

/// Housing transaction and listing counts.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HousingEventCounts {
    pub date: String,
    pub sales: Option<i64>,
    pub new_listings_for_sale: Option<i64>,
    pub new_rental_listings: Option<i64>,
}

/// Housing unit counts by property type.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HousingStock {
    pub date: String,
    pub single_family: Option<i64>,
    pub condo: Option<i64>,
    pub townhouse: Option<i64>,
    pub other: Option<i64>,
    pub all_properties: Option<i64>,
}

/// Housing event prices with statistical breakdowns.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HousingEventPrices {
    pub date: String,
    pub price: Option<PriceStats>,
    pub price_per_square_foot: Option<PriceStats>,
}

/// Price statistics across different event types.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceStats {
    pub median: Option<EventPrices>,
    pub standard_deviation: Option<EventPrices>,
    pub percentile_20th: Option<EventPrices>,
    pub percentile_80th: Option<EventPrices>,
}

/// Price values for each event type.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventPrices {
    pub sales: Option<f64>,
    pub new_listings_for_sale: Option<f64>,
    pub new_rental_listings: Option<f64>,
}

// ============================================================================
// Price Feed
// ============================================================================

/// Price feed data point for trading.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceFeedEntry {
    pub date: String,
    pub price: f64,
    pub price_feed_type: Option<String>,
}

// ============================================================================
// Investor Metrics
// ============================================================================

/// Investor ownership data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InvestorHousingStock {
    pub date: String,
    pub investor_owned_units: Option<i64>,
    pub investor_ownership_pct: Option<f64>,
}

// ============================================================================
// For Sale Metrics
// ============================================================================

/// For-sale inventory metrics.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForSaleInventory {
    pub date: String,
    pub total_inventory: Option<i64>,
    pub median_days_on_market: Option<i64>,
}

// ============================================================================
// Rental Metrics
// ============================================================================

/// Rental market metrics.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RentalMetrics {
    pub date: String,
    /// Annual rental income divided by median sale price.
    pub gross_yield: Option<f64>,
    pub rental_units_concentration: Option<f64>,
}
