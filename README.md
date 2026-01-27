# parcllabs-rust

Rust SDK for the [Parcl Labs API](https://docs.parcllabs.com/) - U.S. housing market data and analytics.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
parcllabs = { git = "https://github.com/ExoMonk/parcllabs-rust" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use parcllabs::{ParclClient, SearchParams, MetricsParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client (reads PARCL_LABS_API_KEY from environment)
    let client = ParclClient::new()?;

    // Search for markets
    let params = SearchParams::new()
        .query("Los Angeles")
        .state("CA")
        .limit(5);
    let markets = client.search().markets(params).await?;

    let la = &markets.items[0];
    println!("Found: {} (parcl_id: {})", la.name, la.parcl_id);

    // Get housing metrics
    let params = MetricsParams::new()
        .limit(12)
        .start_date("2024-01-01");

    let events = client
        .market_metrics()
        .housing_event_counts(la.parcl_id, Some(params))
        .await?;

    for event in &events.items {
        println!(
            "{}: {} sales, {} new listings",
            event.date,
            event.sales.unwrap_or(0),
            event.new_listings_for_sale.unwrap_or(0)
        );
    }

    Ok(())
}
```

## Authentication

Get your API key at [app.parcllabs.com](https://app.parcllabs.com/data-vault).

```bash
# Set environment variable
export PARCL_LABS_API_KEY=your_api_key

# Or use .env file
cp .env.example .env
```

Or pass it directly:

```rust
let client = ParclClient::with_api_key("your_api_key");

// Custom base URL (for testing)
let client = ParclClient::with_config("your_api_key", "https://custom.api.com");
```

## Features

### Search Markets

Find markets by name, state, location type, or region:

```rust
use parcllabs::{SearchParams, LocationType, USRegion, SortBy, SortOrder};

let params = SearchParams::new()
    .query("San Francisco")
    .state("CA")
    .location_type(LocationType::City)
    .region(USRegion::Pacific)
    .sort_by(SortBy::TotalPopulation)
    .sort_order(SortOrder::Desc)
    .limit(10);

let markets = client.search().markets(params).await?;

for market in &markets.items {
    println!("{} ({}): pop {}",
        market.name,
        market.location_type,
        market.total_population.unwrap_or(0)
    );

    if market.has_price_feed() {
        println!("  ^ Price feed available");
    }
}
```

### Market Metrics

Get housing data for a specific market:

```rust
use parcllabs::{MetricsParams, PropertyType};

let parcl_id = 2900078; // Los Angeles

// Housing event counts (sales, listings)
let events = client
    .market_metrics()
    .housing_event_counts(parcl_id, None)
    .await?;

// Housing stock (property type breakdown)
let stock = client
    .market_metrics()
    .housing_stock(parcl_id, None)
    .await?;

// Housing prices filtered by property type
let params = MetricsParams::new()
    .property_type(PropertyType::SingleFamily)
    .start_date("2024-01-01");

let prices = client
    .market_metrics()
    .housing_event_prices(parcl_id, Some(params))
    .await?;
```

### Price Feed

Get price feed data for tradeable markets:

```rust
let feed = client
    .price_feed()
    .history(parcl_id, Some(MetricsParams::new().limit(30)))
    .await?;

for entry in &feed.items {
    println!("{}: ${:.2}", entry.date, entry.price);
}
```

### Auto-Pagination

Automatically fetch all pages of results:

```rust
// Search - fetch all matching markets
let params = SearchParams::new()
    .query("San")
    .state("CA")
    .auto_paginate(true);

let all_markets = client.search().markets(params).await?;
println!("Fetched {} markets", all_markets.items.len());

// Metrics - fetch all historical data
let params = MetricsParams::new()
    .start_date("2020-01-01")
    .auto_paginate(true);

let all_events = client
    .market_metrics()
    .housing_event_counts(parcl_id, Some(params))
    .await?;
```

## Examples

```bash
# Set up your .env file
cp .env.example .env
# Edit .env with your API key

# Run examples
cargo run --example search_markets
cargo run --example market_metrics
```

## API Coverage

### Implemented

| Category | Endpoint | Method |
|----------|----------|--------|
| **Search** | `/v1/search/markets` | `search().markets()` |
| **Market Metrics** | `/v1/market_metrics/{id}/housing_event_counts` | `market_metrics().housing_event_counts()` |
| **Market Metrics** | `/v1/market_metrics/{id}/housing_event_prices` | `market_metrics().housing_event_prices()` |
| **Market Metrics** | `/v1/market_metrics/{id}/housing_stock` | `market_metrics().housing_stock()` |
| **Price Feed** | `/v1/price_feed/{id}/history` | `price_feed().history()` |

### TODO

| Category | Endpoint | Priority |
|----------|----------|----------|
| **Market Metrics** | `/v1/market_metrics/{id}/all_cash` | High |
| **Market Metrics** | `/v1/market_metrics/{id}/housing_event_property_attributes` | Medium |
| **Market Metrics** | `POST /v1/market_metrics/*` (batch endpoints) | Medium |
| **For Sale Metrics** | `/v1/for_sale_market_metrics/{id}/for_sale_inventory` | High |
| **For Sale Metrics** | `/v1/for_sale_market_metrics/{id}/for_sale_inventory_price_changes` | Medium |
| **For Sale Metrics** | `/v1/for_sale_market_metrics/{id}/new_listings_rolling_counts` | Medium |
| **Rental Metrics** | `/v1/rental_market_metrics/{id}/gross_yield` | High |
| **Rental Metrics** | `/v1/rental_market_metrics/{id}/rental_units_concentration` | Medium |
| **Rental Metrics** | `/v1/rental_market_metrics/{id}/new_listings_for_rent_rolling_counts` | Medium |
| **Investor Metrics** | `/v1/investor_metrics/{id}/housing_stock_ownership` | High |
| **Investor Metrics** | `/v1/investor_metrics/{id}/housing_event_counts` | Medium |
| **Investor Metrics** | `/v1/investor_metrics/{id}/housing_event_prices` | Medium |
| **Investor Metrics** | `/v1/investor_metrics/{id}/new_listings_for_sale_rolling_counts` | Medium |
| **Investor Metrics** | `/v1/investor_metrics/{id}/purchase_to_sale_ratio` | Medium |
| **Portfolio Metrics** | `/v1/portfolio_metrics/{id}/sf_housing_event_counts` | Low |
| **Portfolio Metrics** | `/v1/portfolio_metrics/{id}/sf_housing_stock_ownership` | Low |
| **Portfolio Metrics** | `/v1/portfolio_metrics/{id}/sf_new_listings_for_*_rolling_counts` | Low |
| **New Construction** | `/v1/new_construction_metrics/{id}/housing_event_counts` | Medium |
| **New Construction** | `/v1/new_construction_metrics/{id}/housing_event_prices` | Medium |
| **Price Feed** | `/v1/price_feed/{id}/rental_price_feed` | High |
| **Price Feed** | `POST /v1/price_feed/*` (batch endpoints) | Medium |
| **Property API** | `/v1/property_search` | Low |
| **Property API** | `/v2/property_search` | Low |
| **Property API** | `/v1/property_event_history` | Low |
| **Property API** | `/v1/property_search_address` | Low |

### Other TODOs

- [ ] CI/CD pipeline
- [ ] Publish to crates.io
- [ ] Rate limiting / retry logic
- [ ] Response caching
- [ ] Streaming for large datasets

## License

MIT
