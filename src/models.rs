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
    /// Country code (e.g. "US").
    pub country: Option<String>,
    /// Geographic identifier.
    pub geoid: Option<String>,
    /// US Census region.
    pub region: Option<String>,
    /// Whether this market is in the Case-Shiller 10-city index (0 or 1).
    pub case_shiller_10_market: Option<i32>,
    /// Whether this market is in the Case-Shiller 20-city index (0 or 1).
    pub case_shiller_20_market: Option<i32>,
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

/// Portfolio size filter for portfolio metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PortfolioSize {
    Portfolio2To9,
    Portfolio10To99,
    Portfolio100To999,
    Portfolio1000Plus,
    #[default]
    AllPortfolios,
}

impl PortfolioSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Portfolio2To9 => "PORTFOLIO_2_TO_9",
            Self::Portfolio10To99 => "PORTFOLIO_10_TO_99",
            Self::Portfolio100To999 => "PORTFOLIO_100_TO_999",
            Self::Portfolio1000Plus => "PORTFOLIO_1000_PLUS",
            Self::AllPortfolios => "ALL_PORTFOLIOS",
        }
    }
}

impl std::fmt::Display for PortfolioSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Event type filter for property event history queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Sale,
    Listing,
    Rental,
    All,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sale => "SALE",
            Self::Listing => "LISTING",
            Self::Rental => "RENTAL",
            Self::All => "ALL",
        }
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Institutional investor / entity owner name filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityOwnerName {
    Amh,
    Tricon,
    InvitationHomes,
    HomePartnersOfAmerica,
    ProgressResidential,
    FirstkeyHomes,
    Amherst,
    MaymontHomes,
    VinebrookHomes,
    Sfr3,
    MyCommunityHomes,
    Blackstone,
    Bx,
    Opendoor,
    Offerpad,
}

impl EntityOwnerName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Amh => "AMH",
            Self::Tricon => "TRICON",
            Self::InvitationHomes => "INVITATION_HOMES",
            Self::HomePartnersOfAmerica => "HOME_PARTNERS_OF_AMERICA",
            Self::ProgressResidential => "PROGRESS_RESIDENTIAL",
            Self::FirstkeyHomes => "FIRSTKEY_HOMES",
            Self::Amherst => "AMHERST",
            Self::MaymontHomes => "MAYMONT_HOMES",
            Self::VinebrookHomes => "VINEBROOK_HOMES",
            Self::Sfr3 => "SFR3",
            Self::MyCommunityHomes => "MY_COMMUNITY_HOMES",
            Self::Blackstone => "BLACKSTONE",
            Self::Bx => "BX",
            Self::Opendoor => "OPENDOOR",
            Self::Offerpad => "OFFERPAD",
        }
    }
}

impl std::fmt::Display for EntityOwnerName {
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

/// All-cash transaction metrics.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AllCash {
    pub date: String,
    /// Count of all-cash arms-length sales.
    pub count_sales: Option<i64>,
    /// Percentage of arms-length sales completed as all-cash.
    pub pct_sales: Option<f64>,
    /// Count of all-cash transfers across all sale types.
    pub count_transfers: Option<i64>,
    /// Percentage of transfers completed as all-cash.
    pub pct_transfers: Option<f64>,
}

/// Physical attributes of properties involved in housing events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HousingEventPropertyAttributes {
    pub date: String,
    /// Median bedroom count.
    pub beds: Option<i64>,
    /// Median bathroom count.
    pub baths: Option<f64>,
    /// Median square footage.
    pub sqft: Option<i64>,
    /// Median lot size in square feet.
    pub lot_size: Option<i64>,
    /// Median year built.
    pub year_built: Option<i64>,
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

/// Investor housing stock ownership data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InvestorHousingStockOwnership {
    pub date: String,
    /// Count of properties owned by investors.
    #[serde(rename = "count")]
    pub investor_owned_count: Option<i64>,
    /// Percentage of housing stock owned by investors.
    #[serde(rename = "pct_ownership")]
    pub investor_owned_pct: Option<f64>,
}

/// Investor purchase-to-sale ratio data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InvestorPurchaseToSaleRatio {
    pub date: String,
    /// Number of acquisitions by investors.
    pub acquisitions: Option<i64>,
    /// Number of dispositions by investors.
    pub dispositions: Option<i64>,
    /// Ratio of purchases to sales (>1 = net buyer, <1 = net seller).
    pub purchase_to_sale_ratio: Option<f64>,
}

/// Investor housing event counts.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InvestorHousingEventCounts {
    pub date: String,
    /// Count of investor property acquisitions.
    pub acquisitions: Option<i64>,
    /// Count of investor property dispositions.
    pub dispositions: Option<i64>,
    /// Properties newly listed for sale by investors.
    pub new_listings_for_sale: Option<i64>,
    /// Properties newly listed for rent by investors.
    pub new_rental_listings: Option<i64>,
}

/// Rolling counts with multiple time windows.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RollingCounts {
    /// 7-day rolling count.
    pub rolling_7_day: Option<i64>,
    /// 30-day rolling count.
    pub rolling_30_day: Option<i64>,
    /// 60-day rolling count.
    pub rolling_60_day: Option<i64>,
    /// 90-day rolling count.
    pub rolling_90_day: Option<i64>,
}

/// Rolling percentages with multiple time windows.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RollingPercentages {
    /// 7-day rolling percentage.
    pub rolling_7_day: Option<f64>,
    /// 30-day rolling percentage.
    pub rolling_30_day: Option<f64>,
    /// 60-day rolling percentage.
    pub rolling_60_day: Option<f64>,
    /// 90-day rolling percentage.
    pub rolling_90_day: Option<f64>,
}

/// Rolling counts for investor new listings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InvestorNewListingsRollingCounts {
    pub date: String,
    /// Rolling counts of new listings.
    pub count: Option<RollingCounts>,
    /// Percentage of for-sale market by rolling period.
    pub pct_for_sale_market: Option<RollingPercentages>,
}

// ============================================================================
// For Sale Metrics
// ============================================================================

/// For-sale inventory metrics.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForSaleInventory {
    pub date: String,
    /// Total count of properties listed for sale.
    pub for_sale_inventory: Option<i64>,
}

/// For-sale inventory price change metrics.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForSaleInventoryPriceChanges {
    pub date: String,
    /// Count of listings with any price change.
    pub count_price_change: Option<i64>,
    /// Count of listings with price drops.
    pub count_price_drop: Option<i64>,
    /// Median days between price changes.
    #[serde(rename = "median_days_bt_change")]
    pub median_days_bt_price_change: Option<f64>,
    /// Median price change amount.
    pub median_price_change: Option<f64>,
    /// Median percentage price change.
    pub median_pct_price_change: Option<f64>,
    /// Percentage of inventory with price changes.
    #[serde(rename = "pct_inventory_price_change")]
    pub pct_price_change: Option<f64>,
    /// Percentage of inventory with price drops.
    #[serde(rename = "pct_inventory_price_drop")]
    pub pct_price_drop: Option<f64>,
}

/// Rolling counts for new for-sale listings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewListingsRollingCounts {
    pub date: String,
    /// 7-day rolling count.
    #[serde(rename = "rolling_7_day")]
    pub rolling_7_day_count: Option<i64>,
    /// 30-day rolling count.
    #[serde(rename = "rolling_30_day")]
    pub rolling_30_day_count: Option<i64>,
    /// 60-day rolling count.
    #[serde(rename = "rolling_60_day")]
    pub rolling_60_day_count: Option<i64>,
    /// 90-day rolling count.
    #[serde(rename = "rolling_90_day")]
    pub rolling_90_day_count: Option<i64>,
}

// ============================================================================
// Rental Metrics
// ============================================================================

/// Gross rental yield metrics.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrossYield {
    pub date: String,
    /// Annual rental income divided by median sale price.
    pub gross_yield: Option<f64>,
}

/// Rental units concentration metrics.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RentalUnitsConcentration {
    pub date: String,
    /// Percentage of housing stock that are rental units.
    pub rental_units_concentration: Option<f64>,
}

/// Rolling counts for new rental listings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RentalNewListingsRollingCounts {
    pub date: String,
    /// 7-day rolling count.
    #[serde(rename = "rolling_7_day")]
    pub rolling_7_day_count: Option<i64>,
    /// 30-day rolling count.
    #[serde(rename = "rolling_30_day")]
    pub rolling_30_day_count: Option<i64>,
    /// 60-day rolling count.
    #[serde(rename = "rolling_60_day")]
    pub rolling_60_day_count: Option<i64>,
    /// 90-day rolling count.
    #[serde(rename = "rolling_90_day")]
    pub rolling_90_day_count: Option<i64>,
}

// ============================================================================
// Portfolio Metrics
// ============================================================================

/// Count breakdown by portfolio size for housing stock ownership.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PortfolioSizeBreakdown {
    pub portfolio_2_to_9: Option<i64>,
    pub portfolio_10_to_99: Option<i64>,
    pub portfolio_100_to_999: Option<i64>,
    pub portfolio_1000_plus: Option<i64>,
    pub all_portfolios: Option<i64>,
}

/// Percentage breakdown by portfolio size for housing stock ownership.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PortfolioSizePctBreakdown {
    pub portfolio_2_to_9: Option<f64>,
    pub portfolio_10_to_99: Option<f64>,
    pub portfolio_100_to_999: Option<f64>,
    pub portfolio_1000_plus: Option<f64>,
    pub all_portfolios: Option<f64>,
}

/// SF housing stock ownership broken down by portfolio size.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PortfolioStockOwnership {
    pub date: String,
    /// Owned property counts by portfolio size.
    pub count: Option<PortfolioSizeBreakdown>,
    /// Percentage of SF housing stock by portfolio size.
    pub pct_sf_housing_stock: Option<PortfolioSizePctBreakdown>,
}

/// Portfolio holder housing event counts.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PortfolioHousingEventCounts {
    pub date: String,
    pub acquisitions: Option<i64>,
    pub dispositions: Option<i64>,
    pub new_listings_for_sale: Option<i64>,
    pub new_rental_listings: Option<i64>,
    pub transfers: Option<i64>,
}

/// Rolling counts for portfolio new for-sale listings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PortfolioNewListingsRollingCounts {
    pub date: String,
    /// Rolling counts of new for-sale listings.
    pub count: Option<RollingCounts>,
    /// Percentage of SF for-sale market by rolling period.
    pub pct_sf_for_sale_market: Option<RollingPercentages>,
}

/// Rolling counts for portfolio new rental listings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PortfolioRentalListingsRollingCounts {
    pub date: String,
    /// Rolling counts of new rental listings.
    pub count: Option<RollingCounts>,
    /// Percentage of SF rental market by rolling period.
    pub pct_sf_for_rent_market: Option<RollingPercentages>,
}

// ============================================================================
// Property API — Response Models
// ============================================================================

/// API account/credit usage info returned by property endpoints.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountInfo {
    pub est_credits_used: Option<i64>,
    pub est_remaining_credits: Option<i64>,
}

/// Response from `GET /v1/property/search` and `POST /v1/property/search_address`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertySearchResponse {
    pub items: Vec<Property>,
    pub account: Option<AccountInfo>,
}

/// A property returned from the v1 property search endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Property {
    pub parcl_property_id: i64,
    pub address: Option<String>,
    pub unit: Option<String>,
    pub city: Option<String>,
    pub zip_code: Option<String>,
    pub state_abbreviation: Option<String>,
    pub county: Option<String>,
    pub cbsa: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub property_type: Option<String>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<f64>,
    pub square_footage: Option<i64>,
    pub year_built: Option<i32>,
    pub cbsa_parcl_id: Option<i64>,
    pub county_parcl_id: Option<i64>,
    pub city_parcl_id: Option<i64>,
    pub zip_parcl_id: Option<i64>,
    pub event_count: Option<i64>,
    pub event_history_sale_flag: Option<i32>,
    pub event_history_rental_flag: Option<i32>,
    pub event_history_listing_flag: Option<i32>,
    pub current_new_construction_flag: Option<i32>,
    pub current_owner_occupied_flag: Option<i32>,
    pub current_investor_owned_flag: Option<i32>,
    pub current_entity_owner_name: Option<String>,
    pub current_on_market_flag: Option<i32>,
    pub current_on_market_rental_flag: Option<i32>,
    pub record_added_date: Option<String>,
}

/// Response from `POST /v1/property/event_history`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyEventHistoryResponse {
    pub properties: Vec<PropertyWithEvents>,
}

/// A property with its event history.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyWithEvents {
    pub parcl_property_id: i64,
    pub property_metadata: Option<PropertyMetadata>,
    pub events: Option<Vec<PropertyEvent>>,
}

/// Basic property metadata returned with event history.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyMetadata {
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<f64>,
    pub square_footage: Option<i64>,
    pub year_built: Option<i32>,
    pub property_type: Option<String>,
}

/// A single property event (sale, listing, rental).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyEvent {
    pub event_type: Option<String>,
    pub event_name: Option<String>,
    pub event_date: Option<String>,
    pub price: Option<i64>,
    pub entity_owner_name: Option<String>,
    pub investor_flag: Option<i32>,
    pub owner_occupied_flag: Option<i32>,
    pub new_construction_flag: Option<i32>,
    pub record_updated_date: Option<String>,
}

/// Response from `POST /v2/property_search`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyV2SearchResponse {
    pub properties: Vec<PropertyV2>,
}

/// A property returned from the v2 search endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyV2 {
    pub parcl_property_id: i64,
    pub property_metadata: Option<PropertyV2Metadata>,
    pub events: Option<Vec<PropertyV2Event>>,
}

/// Detailed property metadata from v2 search.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyV2Metadata {
    pub bathrooms: Option<f64>,
    pub bedrooms: Option<i32>,
    pub sq_ft: Option<i64>,
    pub year_built: Option<i32>,
    pub property_type: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip5: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub city_name: Option<String>,
    pub county_name: Option<String>,
    pub metro_name: Option<String>,
    pub record_added_date: Option<String>,
    pub current_on_market_flag: Option<i32>,
    pub current_on_market_rental_flag: Option<i32>,
    pub current_new_construction_flag: Option<i32>,
    pub current_owner_occupied_flag: Option<i32>,
    pub current_investor_owned_flag: Option<i32>,
    pub current_entity_owner_name: Option<String>,
}

/// A property event from v2 search (richer than v1).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyV2Event {
    pub event_type: Option<String>,
    pub event_name: Option<String>,
    pub event_date: Option<String>,
    pub entity_owner_name: Option<String>,
    pub true_sale_index: Option<i32>,
    pub price: Option<i64>,
    pub transfer_index: Option<i32>,
    pub investor_flag: Option<i32>,
    pub owner_occupied_flag: Option<i32>,
    pub new_construction_flag: Option<i32>,
    pub current_owner_flag: Option<i32>,
    pub record_updated_date: Option<String>,
}

// ============================================================================
// Property API — Request Bodies
// ============================================================================

/// A single address for the address search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressSearchRequest {
    pub address: String,
    pub city: String,
    pub state_abbreviation: String,
    pub zip_code: String,
}

/// Request body for `POST /v2/property_search`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PropertyV2SearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parcl_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parcl_property_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_coordinates: Option<GeoCoordinates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_filters: Option<PropertyFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filters: Option<V2EventFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_filters: Option<OwnerFilters>,
}

/// Geographic search coordinates for v2 property search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoCoordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub radius_miles: f64,
}

/// Property filters for v2 search request body.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PropertyFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_property_details: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_beds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_beds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_baths: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_baths: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_sqft: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sqft: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_year_built: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_year_built: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_entity_owner_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_owner_occupied_flag: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_investor_owned_flag: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_on_market_flag: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_on_market_rental_flag: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_new_construction_flag: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_record_added_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_record_added_date: Option<String>,
}

/// Event filters for v2 search request body.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct V2EventFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_event_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_event_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_event_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_event_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_events: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_full_event_history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new_construction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_record_updated_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_record_updated_date: Option<String>,
}

/// Owner filters for v2 search request body.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OwnerFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_seller_name: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_current_owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_investor_owned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owner_occupied: Option<bool>,
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
            country: None,
            geoid: None,
            region: None,
            case_shiller_10_market: None,
            case_shiller_20_market: None,
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

    #[test]
    fn all_cash_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "count_sales": 150,
            "pct_sales": 28.5,
            "count_transfers": 200,
            "pct_transfers": 32.1
        }"#;

        let all_cash: AllCash = serde_json::from_str(json).unwrap();
        assert_eq!(all_cash.date, "2024-01-01");
        assert_eq!(all_cash.count_sales, Some(150));
        assert!((all_cash.pct_sales.unwrap() - 28.5).abs() < f64::EPSILON);
        assert_eq!(all_cash.count_transfers, Some(200));
        assert!((all_cash.pct_transfers.unwrap() - 32.1).abs() < f64::EPSILON);
    }

    #[test]
    fn all_cash_deserialize_with_nulls() {
        let json = r#"{
            "date": "2024-01-01",
            "count_sales": null,
            "pct_sales": null,
            "count_transfers": null,
            "pct_transfers": null
        }"#;

        let all_cash: AllCash = serde_json::from_str(json).unwrap();
        assert_eq!(all_cash.date, "2024-01-01");
        assert!(all_cash.count_sales.is_none());
        assert!(all_cash.pct_sales.is_none());
    }

    #[test]
    fn housing_event_property_attributes_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "beds": 3,
            "baths": 2.5,
            "sqft": 1850,
            "lot_size": 6500,
            "year_built": 1995
        }"#;

        let attrs: HousingEventPropertyAttributes = serde_json::from_str(json).unwrap();
        assert_eq!(attrs.date, "2024-01-01");
        assert_eq!(attrs.beds, Some(3));
        assert!((attrs.baths.unwrap() - 2.5).abs() < f64::EPSILON);
        assert_eq!(attrs.sqft, Some(1850));
        assert_eq!(attrs.lot_size, Some(6500));
        assert_eq!(attrs.year_built, Some(1995));
    }

    #[test]
    fn housing_event_property_attributes_deserialize_with_nulls() {
        let json = r#"{
            "date": "2024-01-01",
            "beds": 4,
            "baths": null,
            "sqft": 2000,
            "lot_size": null,
            "year_built": null
        }"#;

        let attrs: HousingEventPropertyAttributes = serde_json::from_str(json).unwrap();
        assert_eq!(attrs.date, "2024-01-01");
        assert_eq!(attrs.beds, Some(4));
        assert!(attrs.baths.is_none());
        assert_eq!(attrs.sqft, Some(2000));
        assert!(attrs.lot_size.is_none());
        assert!(attrs.year_built.is_none());
    }

    #[test]
    fn investor_housing_stock_ownership_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "count": 15000,
            "pct_ownership": 12.5
        }"#;

        let ownership: InvestorHousingStockOwnership = serde_json::from_str(json).unwrap();
        assert_eq!(ownership.date, "2024-01-01");
        assert_eq!(ownership.investor_owned_count, Some(15000));
        assert!((ownership.investor_owned_pct.unwrap() - 12.5).abs() < f64::EPSILON);
    }

    #[test]
    fn investor_purchase_to_sale_ratio_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "acquisitions": 120,
            "dispositions": 80,
            "purchase_to_sale_ratio": 1.5
        }"#;

        let ratio: InvestorPurchaseToSaleRatio = serde_json::from_str(json).unwrap();
        assert_eq!(ratio.date, "2024-01-01");
        assert_eq!(ratio.acquisitions, Some(120));
        assert_eq!(ratio.dispositions, Some(80));
        assert!((ratio.purchase_to_sale_ratio.unwrap() - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn investor_housing_event_counts_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "acquisitions": 100,
            "dispositions": 75,
            "new_listings_for_sale": 50,
            "new_rental_listings": 25
        }"#;

        let counts: InvestorHousingEventCounts = serde_json::from_str(json).unwrap();
        assert_eq!(counts.date, "2024-01-01");
        assert_eq!(counts.acquisitions, Some(100));
        assert_eq!(counts.dispositions, Some(75));
        assert_eq!(counts.new_listings_for_sale, Some(50));
        assert_eq!(counts.new_rental_listings, Some(25));
    }

    #[test]
    fn investor_new_listings_rolling_counts_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "count": {
                "rolling_7_day": 10,
                "rolling_30_day": 45,
                "rolling_60_day": 88,
                "rolling_90_day": 130
            },
            "pct_for_sale_market": {
                "rolling_7_day": 5.2,
                "rolling_30_day": 6.1,
                "rolling_60_day": 5.8,
                "rolling_90_day": 5.5
            }
        }"#;

        let counts: InvestorNewListingsRollingCounts = serde_json::from_str(json).unwrap();
        assert_eq!(counts.date, "2024-01-01");
        let c = counts.count.unwrap();
        assert_eq!(c.rolling_7_day, Some(10));
        assert_eq!(c.rolling_30_day, Some(45));
        assert_eq!(c.rolling_90_day, Some(130));
        let p = counts.pct_for_sale_market.unwrap();
        assert!((p.rolling_7_day.unwrap() - 5.2).abs() < f64::EPSILON);
    }

    #[test]
    fn investor_new_listings_rolling_counts_with_nulls() {
        let json = r#"{
            "date": "2024-01-01",
            "count": {
                "rolling_7_day": null,
                "rolling_30_day": 45,
                "rolling_60_day": null,
                "rolling_90_day": null
            },
            "pct_for_sale_market": null
        }"#;

        let counts: InvestorNewListingsRollingCounts = serde_json::from_str(json).unwrap();
        assert_eq!(counts.date, "2024-01-01");
        let c = counts.count.unwrap();
        assert!(c.rolling_7_day.is_none());
        assert_eq!(c.rolling_30_day, Some(45));
        assert!(c.rolling_60_day.is_none());
        assert!(counts.pct_for_sale_market.is_none());
    }

    #[test]
    fn for_sale_inventory_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "for_sale_inventory": 1250
        }"#;

        let inventory: ForSaleInventory = serde_json::from_str(json).unwrap();
        assert_eq!(inventory.date, "2024-01-01");
        assert_eq!(inventory.for_sale_inventory, Some(1250));
    }

    #[test]
    fn for_sale_inventory_deserialize_with_null() {
        let json = r#"{
            "date": "2024-01-01",
            "for_sale_inventory": null
        }"#;

        let inventory: ForSaleInventory = serde_json::from_str(json).unwrap();
        assert_eq!(inventory.date, "2024-01-01");
        assert!(inventory.for_sale_inventory.is_none());
    }

    #[test]
    fn for_sale_inventory_price_changes_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "count_price_change": 150,
            "count_price_drop": 120,
            "median_days_bt_change": 21.5,
            "median_price_change": -25000.0,
            "median_pct_price_change": -2.5,
            "pct_inventory_price_change": 12.5,
            "pct_inventory_price_drop": 10.2
        }"#;

        let changes: ForSaleInventoryPriceChanges = serde_json::from_str(json).unwrap();
        assert_eq!(changes.date, "2024-01-01");
        assert_eq!(changes.count_price_change, Some(150));
        assert_eq!(changes.count_price_drop, Some(120));
        assert!((changes.median_days_bt_price_change.unwrap() - 21.5).abs() < f64::EPSILON);
        assert!((changes.median_price_change.unwrap() - (-25000.0)).abs() < f64::EPSILON);
        assert!((changes.median_pct_price_change.unwrap() - (-2.5)).abs() < f64::EPSILON);
        assert!((changes.pct_price_change.unwrap() - 12.5).abs() < f64::EPSILON);
        assert!((changes.pct_price_drop.unwrap() - 10.2).abs() < f64::EPSILON);
    }

    #[test]
    fn for_sale_inventory_price_changes_deserialize_with_nulls() {
        let json = r#"{
            "date": "2024-01-01",
            "count_price_change": 100,
            "count_price_drop": null,
            "median_days_bt_change": null,
            "median_price_change": null,
            "pct_inventory_price_change": 8.0,
            "pct_inventory_price_drop": null
        }"#;

        let changes: ForSaleInventoryPriceChanges = serde_json::from_str(json).unwrap();
        assert_eq!(changes.date, "2024-01-01");
        assert_eq!(changes.count_price_change, Some(100));
        assert!(changes.count_price_drop.is_none());
        assert!(changes.median_days_bt_price_change.is_none());
        assert!((changes.pct_price_change.unwrap() - 8.0).abs() < f64::EPSILON);
    }

    #[test]
    fn new_listings_rolling_counts_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "rolling_7_day": 45,
            "rolling_30_day": 180,
            "rolling_60_day": 350,
            "rolling_90_day": 520
        }"#;

        let counts: NewListingsRollingCounts = serde_json::from_str(json).unwrap();
        assert_eq!(counts.date, "2024-01-01");
        assert_eq!(counts.rolling_7_day_count, Some(45));
        assert_eq!(counts.rolling_30_day_count, Some(180));
        assert_eq!(counts.rolling_60_day_count, Some(350));
        assert_eq!(counts.rolling_90_day_count, Some(520));
    }

    #[test]
    fn new_listings_rolling_counts_deserialize_with_nulls() {
        let json = r#"{
            "date": "2024-01-01",
            "rolling_7_day": null,
            "rolling_30_day": 150,
            "rolling_60_day": null,
            "rolling_90_day": null
        }"#;

        let counts: NewListingsRollingCounts = serde_json::from_str(json).unwrap();
        assert_eq!(counts.date, "2024-01-01");
        assert!(counts.rolling_7_day_count.is_none());
        assert_eq!(counts.rolling_30_day_count, Some(150));
        assert!(counts.rolling_60_day_count.is_none());
        assert!(counts.rolling_90_day_count.is_none());
    }

    #[test]
    fn gross_yield_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "gross_yield": 5.25
        }"#;

        let yield_data: GrossYield = serde_json::from_str(json).unwrap();
        assert_eq!(yield_data.date, "2024-01-01");
        assert!((yield_data.gross_yield.unwrap() - 5.25).abs() < f64::EPSILON);
    }

    #[test]
    fn gross_yield_deserialize_with_null() {
        let json = r#"{
            "date": "2024-01-01",
            "gross_yield": null
        }"#;

        let yield_data: GrossYield = serde_json::from_str(json).unwrap();
        assert_eq!(yield_data.date, "2024-01-01");
        assert!(yield_data.gross_yield.is_none());
    }

    #[test]
    fn rental_units_concentration_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "rental_units_concentration": 35.5
        }"#;

        let concentration: RentalUnitsConcentration = serde_json::from_str(json).unwrap();
        assert_eq!(concentration.date, "2024-01-01");
        assert!((concentration.rental_units_concentration.unwrap() - 35.5).abs() < f64::EPSILON);
    }

    #[test]
    fn rental_new_listings_rolling_counts_deserialize() {
        let json = r#"{
            "date": "2024-01-01",
            "rolling_7_day": 25,
            "rolling_30_day": 100,
            "rolling_60_day": 200,
            "rolling_90_day": 300
        }"#;

        let counts: RentalNewListingsRollingCounts = serde_json::from_str(json).unwrap();
        assert_eq!(counts.date, "2024-01-01");
        assert_eq!(counts.rolling_7_day_count, Some(25));
        assert_eq!(counts.rolling_30_day_count, Some(100));
        assert_eq!(counts.rolling_60_day_count, Some(200));
        assert_eq!(counts.rolling_90_day_count, Some(300));
    }

    #[test]
    fn portfolio_size_as_str() {
        assert_eq!(PortfolioSize::Portfolio2To9.as_str(), "PORTFOLIO_2_TO_9");
        assert_eq!(PortfolioSize::Portfolio10To99.as_str(), "PORTFOLIO_10_TO_99");
        assert_eq!(
            PortfolioSize::Portfolio100To999.as_str(),
            "PORTFOLIO_100_TO_999"
        );
        assert_eq!(
            PortfolioSize::Portfolio1000Plus.as_str(),
            "PORTFOLIO_1000_PLUS"
        );
        assert_eq!(PortfolioSize::AllPortfolios.as_str(), "ALL_PORTFOLIOS");
    }

    #[test]
    fn portfolio_size_default() {
        assert_eq!(PortfolioSize::default(), PortfolioSize::AllPortfolios);
    }

    #[test]
    fn portfolio_size_display() {
        assert_eq!(
            format!("{}", PortfolioSize::Portfolio2To9),
            "PORTFOLIO_2_TO_9"
        );
        assert_eq!(
            format!("{}", PortfolioSize::AllPortfolios),
            "ALL_PORTFOLIOS"
        );
    }

    #[test]
    fn market_deserialize_with_new_fields() {
        let json = r#"{
            "parcl_id": 2900078,
            "name": "Los Angeles",
            "state_abbreviation": "CA",
            "state_fips_code": "06",
            "location_type": "CBSA",
            "total_population": 13000000,
            "median_income": 89000,
            "parcl_exchange_market": 1,
            "pricefeed_market": 1,
            "country": "US",
            "geoid": "31080",
            "region": "PACIFIC",
            "case_shiller_10_market": 1,
            "case_shiller_20_market": 1
        }"#;

        let market: Market = serde_json::from_str(json).unwrap();
        assert_eq!(market.country, Some("US".into()));
        assert_eq!(market.geoid, Some("31080".into()));
        assert_eq!(market.region, Some("PACIFIC".into()));
        assert_eq!(market.case_shiller_10_market, Some(1));
        assert_eq!(market.case_shiller_20_market, Some(1));
    }

    #[test]
    fn market_deserialize_without_new_fields() {
        let json = r#"{
            "parcl_id": 123,
            "name": "Test",
            "location_type": "CITY"
        }"#;

        let market: Market = serde_json::from_str(json).unwrap();
        assert!(market.country.is_none());
        assert!(market.geoid.is_none());
        assert!(market.region.is_none());
        assert!(market.case_shiller_10_market.is_none());
        assert!(market.case_shiller_20_market.is_none());
    }

    #[test]
    fn portfolio_stock_ownership_deserialize() {
        let json = r#"{
            "date": "2025-12-01",
            "count": {
                "portfolio_2_to_9": 128469,
                "portfolio_10_to_99": 26833,
                "portfolio_100_to_999": 20128,
                "portfolio_1000_plus": 52150,
                "all_portfolios": 227580
            },
            "pct_sf_housing_stock": {
                "portfolio_2_to_9": 7.46,
                "portfolio_10_to_99": 1.56,
                "portfolio_100_to_999": 1.17,
                "portfolio_1000_plus": 3.03,
                "all_portfolios": 13.22
            }
        }"#;

        let ownership: PortfolioStockOwnership = serde_json::from_str(json).unwrap();
        assert_eq!(ownership.date, "2025-12-01");
        let c = ownership.count.unwrap();
        assert_eq!(c.portfolio_2_to_9, Some(128469));
        assert_eq!(c.all_portfolios, Some(227580));
        let p = ownership.pct_sf_housing_stock.unwrap();
        assert!((p.all_portfolios.unwrap() - 13.22).abs() < f64::EPSILON);
    }

    #[test]
    fn portfolio_housing_event_counts_deserialize() {
        let json = r#"{
            "date": "2025-12-01",
            "acquisitions": 229,
            "dispositions": 167,
            "new_listings_for_sale": 152,
            "new_rental_listings": 1685,
            "transfers": 253
        }"#;

        let counts: PortfolioHousingEventCounts = serde_json::from_str(json).unwrap();
        assert_eq!(counts.date, "2025-12-01");
        assert_eq!(counts.acquisitions, Some(229));
        assert_eq!(counts.dispositions, Some(167));
        assert_eq!(counts.transfers, Some(253));
    }

    #[test]
    fn portfolio_new_listings_rolling_counts_deserialize() {
        let json = r#"{
            "date": "2026-01-26",
            "count": {
                "rolling_7_day": 259,
                "rolling_30_day": 1286,
                "rolling_60_day": 2375,
                "rolling_90_day": 3815
            },
            "pct_sf_for_sale_market": {
                "rolling_7_day": 18.05,
                "rolling_30_day": 18.46,
                "rolling_60_day": 19.69,
                "rolling_90_day": 19.41
            }
        }"#;

        let data: PortfolioNewListingsRollingCounts = serde_json::from_str(json).unwrap();
        assert_eq!(data.date, "2026-01-26");
        let c = data.count.unwrap();
        assert_eq!(c.rolling_7_day, Some(259));
        assert_eq!(c.rolling_90_day, Some(3815));
        let p = data.pct_sf_for_sale_market.unwrap();
        assert!((p.rolling_7_day.unwrap() - 18.05).abs() < f64::EPSILON);
    }

    #[test]
    fn portfolio_rental_listings_rolling_counts_deserialize() {
        let json = r#"{
            "date": "2026-01-26",
            "count": {
                "rolling_7_day": 575,
                "rolling_30_day": 3038,
                "rolling_60_day": 6613,
                "rolling_90_day": 10697
            },
            "pct_sf_for_rent_market": {
                "rolling_7_day": 45.93,
                "rolling_30_day": 45.04,
                "rolling_60_day": 46.42,
                "rolling_90_day": 46.4
            }
        }"#;

        let data: PortfolioRentalListingsRollingCounts = serde_json::from_str(json).unwrap();
        assert_eq!(data.date, "2026-01-26");
        let c = data.count.unwrap();
        assert_eq!(c.rolling_7_day, Some(575));
        let p = data.pct_sf_for_rent_market.unwrap();
        assert!((p.rolling_30_day.unwrap() - 45.04).abs() < f64::EPSILON);
    }

    // Property API tests

    #[test]
    fn event_type_as_str() {
        assert_eq!(EventType::Sale.as_str(), "SALE");
        assert_eq!(EventType::Listing.as_str(), "LISTING");
        assert_eq!(EventType::Rental.as_str(), "RENTAL");
        assert_eq!(EventType::All.as_str(), "ALL");
    }

    #[test]
    fn event_type_display() {
        assert_eq!(format!("{}", EventType::Sale), "SALE");
    }

    #[test]
    fn entity_owner_name_as_str() {
        assert_eq!(EntityOwnerName::Amh.as_str(), "AMH");
        assert_eq!(EntityOwnerName::InvitationHomes.as_str(), "INVITATION_HOMES");
        assert_eq!(EntityOwnerName::HomePartnersOfAmerica.as_str(), "HOME_PARTNERS_OF_AMERICA");
        assert_eq!(EntityOwnerName::Offerpad.as_str(), "OFFERPAD");
    }

    #[test]
    fn entity_owner_name_display() {
        assert_eq!(format!("{}", EntityOwnerName::Tricon), "TRICON");
    }

    #[test]
    fn property_deserialize() {
        let json = r#"{
            "parcl_property_id": 63325076,
            "address": "1225 W SCHOOL ST",
            "unit": null,
            "city": "CHICAGO",
            "zip_code": "60657",
            "state_abbreviation": "IL",
            "county": "Cook County",
            "cbsa": "Chicago-Naperville-Elgin",
            "latitude": 41.941385,
            "longitude": -87.660019,
            "property_type": "SINGLE_FAMILY",
            "bedrooms": 4,
            "bathrooms": 3.0,
            "square_footage": 5500,
            "year_built": 2024,
            "event_count": 5,
            "event_history_sale_flag": 1,
            "current_on_market_flag": 0,
            "record_added_date": "2024-12-13"
        }"#;

        let prop: Property = serde_json::from_str(json).unwrap();
        assert_eq!(prop.parcl_property_id, 63325076);
        assert_eq!(prop.address, Some("1225 W SCHOOL ST".into()));
        assert!(prop.unit.is_none());
        assert_eq!(prop.bedrooms, Some(4));
        assert!((prop.bathrooms.unwrap() - 3.0).abs() < f64::EPSILON);
        assert_eq!(prop.square_footage, Some(5500));
        assert_eq!(prop.event_history_sale_flag, Some(1));
        assert_eq!(prop.current_on_market_flag, Some(0));
    }

    #[test]
    fn property_search_response_deserialize() {
        let json = r#"{
            "items": [
                {
                    "parcl_property_id": 123,
                    "address": "456 Main St",
                    "property_type": "CONDO"
                }
            ],
            "account": {
                "est_credits_used": 1,
                "est_remaining_credits": 9999
            }
        }"#;

        let resp: PropertySearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].parcl_property_id, 123);
        let acct = resp.account.unwrap();
        assert_eq!(acct.est_credits_used, Some(1));
        assert_eq!(acct.est_remaining_credits, Some(9999));
    }

    #[test]
    fn property_event_history_response_deserialize() {
        let json = r#"{
            "properties": [
                {
                    "parcl_property_id": 63325076,
                    "property_metadata": {
                        "address": "1225 W SCHOOL ST",
                        "city": "CHICAGO",
                        "state": "IL",
                        "zip": "60657",
                        "bedrooms": 4,
                        "bathrooms": 3.0,
                        "square_footage": 5500,
                        "year_built": 2024,
                        "property_type": "SINGLE_FAMILY"
                    },
                    "events": [
                        {
                            "event_type": "SALE",
                            "event_name": "SOLD",
                            "event_date": "2024-12-20",
                            "price": 2645000,
                            "entity_owner_name": null,
                            "investor_flag": 0,
                            "owner_occupied_flag": 1,
                            "new_construction_flag": 1,
                            "record_updated_date": "2025-07-23"
                        }
                    ]
                }
            ]
        }"#;

        let resp: PropertyEventHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.properties.len(), 1);
        let prop = &resp.properties[0];
        assert_eq!(prop.parcl_property_id, 63325076);
        let meta = prop.property_metadata.as_ref().unwrap();
        assert_eq!(meta.city, Some("CHICAGO".into()));
        assert_eq!(meta.bedrooms, Some(4));
        let events = prop.events.as_ref().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_name, Some("SOLD".into()));
        assert_eq!(events[0].price, Some(2645000));
    }

    #[test]
    fn property_v2_search_response_deserialize() {
        let json = r#"{
            "properties": [
                {
                    "parcl_property_id": 63325076,
                    "property_metadata": {
                        "bathrooms": 6.0,
                        "bedrooms": 4,
                        "sq_ft": 5500,
                        "year_built": 2024,
                        "property_type": "SINGLE_FAMILY",
                        "address1": "1225 W SCHOOL ST",
                        "city": "CHICAGO",
                        "state": "IL",
                        "zip5": "60657",
                        "latitude": 41.941385,
                        "longitude": -87.660019,
                        "city_name": "Chicago City",
                        "county_name": "Cook County",
                        "current_on_market_flag": 0,
                        "current_new_construction_flag": 1
                    },
                    "events": [
                        {
                            "event_type": "SALE",
                            "event_name": "SOLD",
                            "event_date": "2024-12-20",
                            "price": 2645000,
                            "true_sale_index": 3,
                            "transfer_index": 3,
                            "investor_flag": 0,
                            "owner_occupied_flag": 1,
                            "new_construction_flag": 1,
                            "current_owner_flag": 1,
                            "record_updated_date": "2025-07-23"
                        }
                    ]
                }
            ]
        }"#;

        let resp: PropertyV2SearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.properties.len(), 1);
        let prop = &resp.properties[0];
        let meta = prop.property_metadata.as_ref().unwrap();
        assert_eq!(meta.sq_ft, Some(5500));
        assert_eq!(meta.address1, Some("1225 W SCHOOL ST".into()));
        assert_eq!(meta.current_new_construction_flag, Some(1));
        let events = prop.events.as_ref().unwrap();
        assert_eq!(events[0].true_sale_index, Some(3));
        assert_eq!(events[0].current_owner_flag, Some(1));
    }

    #[test]
    fn address_search_request_serialize() {
        let req = AddressSearchRequest {
            address: "1225 W SCHOOL ST".into(),
            city: "CHICAGO".into(),
            state_abbreviation: "IL".into(),
            zip_code: "60657".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["address"], "1225 W SCHOOL ST");
        assert_eq!(json["state_abbreviation"], "IL");
    }

    #[test]
    fn property_v2_search_request_serialize_minimal() {
        let req = PropertyV2SearchRequest {
            parcl_ids: Some(vec![5387853]),
            ..Default::default()
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["parcl_ids"], serde_json::json!([5387853]));
        assert!(json.get("property_filters").is_none());
        assert!(json.get("event_filters").is_none());
    }

    #[test]
    fn property_v2_search_request_serialize_full() {
        let req = PropertyV2SearchRequest {
            parcl_ids: Some(vec![5387853]),
            property_filters: Some(PropertyFilters {
                include_property_details: Some(true),
                property_types: Some(vec!["SINGLE_FAMILY".into()]),
                min_beds: Some(3),
                ..Default::default()
            }),
            event_filters: Some(V2EventFilters {
                include_events: Some(true),
                event_names: Some(vec!["SOLD".into()]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["property_filters"]["min_beds"], 3);
        assert_eq!(json["property_filters"]["include_property_details"], true);
        assert_eq!(json["event_filters"]["include_events"], true);
        assert!(json.get("owner_filters").is_none());
    }
}
