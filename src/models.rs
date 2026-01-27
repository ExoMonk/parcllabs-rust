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
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    /// Whether this market is tradeable on the Parcl exchange (0 or 1).
    pub parcl_exchange_market: Option<i32>,
    /// Whether this market has price feed data (0 or 1).
    pub pricefeed_market: Option<i32>,
}

impl Market {
    /// Returns true if this market is tradeable on the Parcl exchange.
    pub fn is_exchange_market(&self) -> bool {
        self.parcl_exchange_market == Some(1)
    }

    /// Returns true if this market has price feed data.
    pub fn has_price_feed(&self) -> bool {
        self.pricefeed_market == Some(1)
    }
}

/// Location type filter for market search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationType {
    County,
    City,
    Zip5,
    Cdp,
    Village,
    Town,
    Cbsa,
    All,
}

impl LocationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::County => "COUNTY",
            Self::City => "CITY",
            Self::Zip5 => "ZIP5",
            Self::Cdp => "CDP",
            Self::Village => "VILLAGE",
            Self::Town => "TOWN",
            Self::Cbsa => "CBSA",
            Self::All => "ALL",
        }
    }
}

impl std::fmt::Display for LocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// US region filter for market search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum USRegion {
    EastNorthCentral,
    EastSouthCentral,
    MiddleAtlantic,
    Mountain,
    NewEngland,
    Pacific,
    SouthAtlantic,
    WestNorthCentral,
    WestSouthCentral,
    All,
}

impl USRegion {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EastNorthCentral => "EAST_NORTH_CENTRAL",
            Self::EastSouthCentral => "EAST_SOUTH_CENTRAL",
            Self::MiddleAtlantic => "MIDDLE_ATLANTIC",
            Self::Mountain => "MOUNTAIN",
            Self::NewEngland => "NEW_ENGLAND",
            Self::Pacific => "PACIFIC",
            Self::SouthAtlantic => "SOUTH_ATLANTIC",
            Self::WestNorthCentral => "WEST_NORTH_CENTRAL",
            Self::WestSouthCentral => "WEST_SOUTH_CENTRAL",
            Self::All => "ALL",
        }
    }
}

impl std::fmt::Display for USRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Sort field for market search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    TotalPopulation,
    MedianIncome,
    CaseShiller20Market,
    CaseShiller10Market,
    PricefeedMarket,
    ParclExchangeMarket,
}

impl SortBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TotalPopulation => "TOTAL_POPULATION",
            Self::MedianIncome => "MEDIAN_INCOME",
            Self::CaseShiller20Market => "CASE_SHILLER_20_MARKET",
            Self::CaseShiller10Market => "CASE_SHILLER_10_MARKET",
            Self::PricefeedMarket => "PRICEFEED_MARKET",
            Self::ParclExchangeMarket => "PARCL_EXCHANGE_MARKET",
        }
    }
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Sort order for market search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Property type filter for market metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropertyType {
    SingleFamily,
    Condo,
    Townhouse,
    Other,
    #[default]
    AllProperties,
}

impl PropertyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingleFamily => "SINGLE_FAMILY",
            Self::Condo => "CONDO",
            Self::Townhouse => "TOWNHOUSE",
            Self::Other => "OTHER",
            Self::AllProperties => "ALL_PROPERTIES",
        }
    }
}

impl std::fmt::Display for PropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_market(exchange: Option<i32>, pricefeed: Option<i32>) -> Market {
        Market {
            parcl_id: 123,
            name: "Test Market".into(),
            state_abbreviation: Some("CA".into()),
            state_fips_code: Some("06".into()),
            location_type: "CITY".into(),
            total_population: Some(100_000),
            median_income: Some(75_000),
            parcl_exchange_market: exchange,
            pricefeed_market: pricefeed,
        }
    }

    #[test]
    fn market_is_exchange_market() {
        assert!(sample_market(Some(1), None).is_exchange_market());
        assert!(!sample_market(Some(0), None).is_exchange_market());
        assert!(!sample_market(None, None).is_exchange_market());
    }

    #[test]
    fn market_has_price_feed() {
        assert!(sample_market(None, Some(1)).has_price_feed());
        assert!(!sample_market(None, Some(0)).has_price_feed());
        assert!(!sample_market(None, None).has_price_feed());
    }

    #[test]
    fn location_type_as_str() {
        assert_eq!(LocationType::County.as_str(), "COUNTY");
        assert_eq!(LocationType::City.as_str(), "CITY");
        assert_eq!(LocationType::Zip5.as_str(), "ZIP5");
        assert_eq!(LocationType::Cbsa.as_str(), "CBSA");
        assert_eq!(LocationType::All.as_str(), "ALL");
    }

    #[test]
    fn location_type_display() {
        assert_eq!(format!("{}", LocationType::City), "CITY");
        assert_eq!(format!("{}", LocationType::County), "COUNTY");
    }

    #[test]
    fn us_region_as_str() {
        assert_eq!(USRegion::Pacific.as_str(), "PACIFIC");
        assert_eq!(USRegion::Mountain.as_str(), "MOUNTAIN");
        assert_eq!(USRegion::NewEngland.as_str(), "NEW_ENGLAND");
        assert_eq!(USRegion::EastNorthCentral.as_str(), "EAST_NORTH_CENTRAL");
    }

    #[test]
    fn sort_by_as_str() {
        assert_eq!(SortBy::TotalPopulation.as_str(), "TOTAL_POPULATION");
        assert_eq!(SortBy::MedianIncome.as_str(), "MEDIAN_INCOME");
        assert_eq!(SortBy::PricefeedMarket.as_str(), "PRICEFEED_MARKET");
    }

    #[test]
    fn sort_order_as_str() {
        assert_eq!(SortOrder::Asc.as_str(), "ASC");
        assert_eq!(SortOrder::Desc.as_str(), "DESC");
    }

    #[test]
    fn property_type_as_str() {
        assert_eq!(PropertyType::SingleFamily.as_str(), "SINGLE_FAMILY");
        assert_eq!(PropertyType::Condo.as_str(), "CONDO");
        assert_eq!(PropertyType::AllProperties.as_str(), "ALL_PROPERTIES");
    }

    #[test]
    fn property_type_default() {
        assert_eq!(PropertyType::default(), PropertyType::AllProperties);
    }

    #[test]
    fn pagination_links_default() {
        let links = PaginationLinks::default();
        assert!(links.first.is_none());
        assert!(links.next.is_none());
        assert!(links.prev.is_none());
        assert!(links.last.is_none());
    }

    #[test]
    fn market_deserialize() {
        let json = r#"{
            "parcl_id": 2900078,
            "name": "Los Angeles",
            "state_abbreviation": "CA",
            "state_fips_code": "06",
            "location_type": "CBSA",
            "total_population": 13000000,
            "median_income": 89000,
            "parcl_exchange_market": 1,
            "pricefeed_market": 1
        }"#;

        let market: Market = serde_json::from_str(json).unwrap();
        assert_eq!(market.parcl_id, 2900078);
        assert_eq!(market.name, "Los Angeles");
        assert!(market.is_exchange_market());
        assert!(market.has_price_feed());
    }

    #[test]
    fn housing_event_counts_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "sales": 1500,
            "new_listings_for_sale": 2000,
            "new_rental_listings": 500
        }"#;

        let counts: HousingEventCounts = serde_json::from_str(json).unwrap();
        assert_eq!(counts.date, "2024-01-01");
        assert_eq!(counts.sales, Some(1500));
        assert_eq!(counts.new_listings_for_sale, Some(2000));
    }

    #[test]
    fn housing_stock_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "single_family": 100000,
            "condo": 50000,
            "townhouse": 10000,
            "other": 5000,
            "all_properties": 165000
        }"#;

        let stock: HousingStock = serde_json::from_str(json).unwrap();
        assert_eq!(stock.all_properties, Some(165000));
        assert_eq!(stock.single_family, Some(100000));
    }

    #[test]
    fn price_feed_entry_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "price": 750000.50,
            "price_feed_type": "daily"
        }"#;

        let entry: PriceFeedEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.date, "2024-01-01");
        assert!((entry.price - 750000.50).abs() < f64::EPSILON);
        assert_eq!(entry.price_feed_type, Some("daily".into()));
    }
}
