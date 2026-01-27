use serde::{Deserialize, Serialize};

/// Paginated response wrapper
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub limit: u64,
    pub offset: u64,
    pub links: PaginationLinks,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationLinks {
    pub first: Option<String>,
    pub next: Option<String>,
    pub prev: Option<String>,
    pub last: Option<String>,
}

// ============================================================================
// Search Models
// ============================================================================

/// Market information returned from search
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Market {
    pub parcl_id: i64,
    pub name: String,
    pub state_abbreviation: Option<String>,
    pub state_fips_code: Option<String>,
    pub location_type: String,
    pub total_population: Option<i64>,
    pub median_income: Option<i64>,
    pub parcl_exchange_market: Option<i32>,
    pub pricefeed_market: Option<i32>,
}

/// Location type filter for market search
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
            LocationType::City => "CITY",
            LocationType::County => "COUNTY",
            LocationType::Zip => "ZIP",
            LocationType::State => "STATE",
            LocationType::Metro => "METRO",
            LocationType::Region => "REGION",
            LocationType::CensusPlace => "CENSUS_PLACE",
            LocationType::National => "NATIONAL",
        }
    }
}

// ============================================================================
// Market Metrics Models
// ============================================================================

/// Housing event counts for a market
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HousingEventCounts {
    pub parcl_id: i64,
    pub date: String,
    #[serde(default)]
    pub sales: Option<i64>,
    #[serde(default)]
    pub new_listings_for_sale: Option<i64>,
    #[serde(default)]
    pub new_rental_listings: Option<i64>,
}

/// Housing stock data for a market
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HousingStock {
    pub parcl_id: i64,
    pub date: String,
    #[serde(default)]
    pub single_family: Option<i64>,
    #[serde(default)]
    pub condo: Option<i64>,
    #[serde(default)]
    pub townhouse: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
}

/// Housing event prices for a market
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HousingEventPrices {
    pub parcl_id: i64,
    pub date: String,
    #[serde(default)]
    pub median_sale_price: Option<f64>,
    #[serde(default)]
    pub median_list_price: Option<f64>,
    #[serde(default)]
    pub median_rental_price: Option<f64>,
}

// ============================================================================
// Price Feed Models
// ============================================================================

/// Price feed data point
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceFeedEntry {
    pub parcl_id: i64,
    pub date: String,
    pub price: f64,
    #[serde(default)]
    pub price_feed_type: Option<String>,
}

// ============================================================================
// Investor Metrics Models
// ============================================================================

/// Investor housing stock ownership
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InvestorHousingStock {
    pub parcl_id: i64,
    pub date: String,
    #[serde(default)]
    pub investor_owned_units: Option<i64>,
    #[serde(default)]
    pub investor_ownership_pct: Option<f64>,
}

// ============================================================================
// For Sale Metrics Models
// ============================================================================

/// For sale inventory metrics
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForSaleInventory {
    pub parcl_id: i64,
    pub date: String,
    #[serde(default)]
    pub total_inventory: Option<i64>,
    #[serde(default)]
    pub median_days_on_market: Option<i64>,
}

// ============================================================================
// Rental Metrics Models
// ============================================================================

/// Rental market metrics
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RentalMetrics {
    pub parcl_id: i64,
    pub date: String,
    #[serde(default)]
    pub gross_yield: Option<f64>,
    #[serde(default)]
    pub rental_units_concentration: Option<f64>,
}
