# parcllabs-rust

[![CI](https://github.com/ExoMonk/parcllabs-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/ExoMonk/parcllabs-rust/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/parcllabs.svg)](https://crates.io/crates/parcllabs)
[![Documentation](https://docs.rs/parcllabs/badge.svg)](https://docs.rs/parcllabs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.83.0-blue.svg)](https://blog.rust-lang.org/2024/11/28/Rust-1.83.0.html)

Rust SDK for the [Parcl Labs API](https://docs.parcllabs.com/) — real-time U.S. housing market data covering 70,000+ markets. 53 endpoints across 9 services with automatic pagination, retry logic, and credit tracking.

## Installation

```toml
[dependencies]
parcllabs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use parcllabs::{ParclClient, SearchParams, MetricsParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ParclClient::new()?; // reads PARCL_LABS_API_KEY env var

    // Search for markets
    let params = SearchParams::new().query("Los Angeles").state("CA").limit(5);
    let markets = client.search().markets(params).await?;
    let la = &markets.items[0];

    // Get housing metrics
    let params = MetricsParams::new().limit(12).start_date("2024-01-01");
    let events = client.market_metrics()
        .housing_event_counts(la.parcl_id, Some(params))
        .await?;

    for event in &events.items {
        println!("{}: {} sales", event.date, event.sales.unwrap_or(0));
    }

    // Check credit usage
    println!("Credits used: {}", client.session_credits_used());

    Ok(())
}
```

## Authentication

```bash
export PARCL_LABS_API_KEY=your_api_key  # from dashboard.parcllabs.com
```

```rust
// From environment
let client = ParclClient::new()?;

// Explicit key
let client = ParclClient::with_api_key("your_api_key");

// Custom base URL
let client = ParclClient::with_config("your_api_key", "https://custom.api.com");
```

## Services

All metric services follow the same pattern: GET for a single market by `parcl_id`, batch POST for multiple markets at once.

### Search

```rust
use parcllabs::{SearchParams, LocationType, SortBy, SortOrder};

let params = SearchParams::new()
    .query("San Francisco")
    .state("CA")
    .location_type(LocationType::City)
    .sort_by(SortBy::TotalPopulation)
    .sort_order(SortOrder::Desc)
    .limit(10);

let markets = client.search().markets(params).await?;
```

### Market Metrics

```rust
use parcllabs::{MetricsParams, PropertyType};

let params = MetricsParams::new()
    .property_type(PropertyType::SingleFamily)
    .start_date("2024-01-01");

let events = client.market_metrics().housing_event_counts(parcl_id, Some(params)).await?;
let prices = client.market_metrics().housing_event_prices(parcl_id, None).await?;
let stock  = client.market_metrics().housing_stock(parcl_id, None).await?;
let cash   = client.market_metrics().all_cash(parcl_id, None).await?;
let attrs  = client.market_metrics().housing_event_property_attributes(parcl_id, None).await?;
```

### Investor Metrics

```rust
let ownership = client.investor_metrics().housing_stock_ownership(parcl_id, None).await?;
let ratio     = client.investor_metrics().purchase_to_sale_ratio(parcl_id, None).await?;
let counts    = client.investor_metrics().housing_event_counts(parcl_id, None).await?;
let prices    = client.investor_metrics().housing_event_prices(parcl_id, None).await?;
let listings  = client.investor_metrics().new_listings_for_sale_rolling_counts(parcl_id, None).await?;
```

### For-Sale Metrics

```rust
let inventory     = client.for_sale_metrics().for_sale_inventory(parcl_id, None).await?;
let price_changes = client.for_sale_metrics().for_sale_inventory_price_changes(parcl_id, None).await?;
let new_listings  = client.for_sale_metrics().new_listings_rolling_counts(parcl_id, None).await?;
```

### Rental Metrics

```rust
let yields        = client.rental_metrics().gross_yield(parcl_id, None).await?;
let concentration = client.rental_metrics().rental_units_concentration(parcl_id, None).await?;
let new_rentals   = client.rental_metrics().new_listings_for_rent_rolling_counts(parcl_id, None).await?;
```

### Price Feed

```rust
let sale_feed   = client.price_feed().history(parcl_id, None).await?;
let rental_feed = client.price_feed().rental_history(parcl_id, None).await?;
```

### New Construction Metrics

```rust
use parcllabs::NewConstructionMetricsParams;

let counts = client.new_construction_metrics().housing_event_counts(parcl_id, None).await?;
let prices = client.new_construction_metrics().housing_event_prices(parcl_id, None).await?;
```

### Portfolio Metrics

Track activity by portfolio size (2-9 units, 10-99, 100-999, 1000+).

```rust
use parcllabs::{PortfolioMetricsParams, PortfolioSize};

let params = PortfolioMetricsParams::new()
    .portfolio_size(PortfolioSize::Portfolio1000Plus);

let ownership = client.portfolio_metrics().sf_housing_stock_ownership(parcl_id, Some(params)).await?;
let events    = client.portfolio_metrics().sf_housing_event_counts(parcl_id, None).await?;
let sales     = client.portfolio_metrics().sf_new_listings_for_sale_rolling_counts(parcl_id, None).await?;
let rentals   = client.portfolio_metrics().sf_new_listings_for_rent_rolling_counts(parcl_id, None).await?;
```

### Property API

Search individual properties with granular filters.

```rust
use parcllabs::{PropertySearchParams, EventHistoryParams};

// Search properties (GET v1)
let params = PropertySearchParams::new(parcl_id)
    .property_type(PropertyType::SingleFamily)
    .limit(50);
let props = client.property().search(params).await?;

// Search by address (POST v1)
let addresses = vec!["123 Main St, Los Angeles, CA 90001".to_string()];
let results = client.property().search_by_address(parcl_id, addresses).await?;

// Event history (POST v1)
let params = EventHistoryParams::new(parcl_id)
    .parcl_property_ids(vec![12345, 67890]);
let history = client.property().event_history(params).await?;
```

## Batch Queries

Every metric endpoint has a `batch_*` variant that accepts multiple `parcl_id`s in a single request. Each response item includes `parcl_id` for identification.

```rust
let ids = vec![2900187, 2900078]; // LA and NYC
let params = MetricsParams::new().limit(5);

let resp = client.market_metrics()
    .batch_housing_event_counts(ids, Some(params))
    .await?;

for item in &resp.items {
    println!("parcl_id: {:?}, date: {}, sales: {:?}",
        item.parcl_id, item.date, item.sales);
}
```

All services support batch: `batch_housing_stock()`, `batch_all_cash()`, `batch_for_sale_inventory()`, `batch_gross_yield()`, `batch_history()`, etc.

## Retry & Rate Limiting

Automatic retry with exponential backoff on HTTP 429 responses. Configurable via `RetryConfig`.

```rust
use parcllabs::RetryConfig;

// Default: 3 retries, 1s initial backoff (doubles each attempt)
let client = ParclClient::new()?;

// Custom retry config
let client = ParclClient::with_api_key("key")
    .with_retry_config(RetryConfig {
        max_retries: 5,
        initial_backoff_ms: 500,
    });

// Disable retries
let client = ParclClient::with_api_key("key")
    .with_retry_config(RetryConfig {
        max_retries: 0,
        initial_backoff_ms: 0,
    });
```

If all retries are exhausted, returns `ParclError::RateLimited`.

## Credit Tracking

API credit usage is tracked automatically from response bodies.

```rust
let client = ParclClient::new()?;

// Make some requests...
let _ = client.market_metrics().housing_event_counts(parcl_id, None).await?;
let _ = client.market_metrics().housing_stock(parcl_id, None).await?;

// Session-level credit tracking
println!("Credits used this session: {}", client.session_credits_used());
println!("Remaining credits: {}", client.remaining_credits());

// Or get both at once
let usage = client.account_info();
println!("{:?}", usage);
```

Per-response credit info is also available on every response:

```rust
let resp = client.market_metrics().housing_event_counts(parcl_id, None).await?;
if let Some(account) = &resp.account {
    println!("This request used {} credits", account.est_credits_used.unwrap_or(0));
}
```

## Auto-Pagination

All services support transparent pagination:

```rust
let params = MetricsParams::new()
    .start_date("2020-01-01")
    .auto_paginate(true);

let history = client.market_metrics()
    .housing_event_counts(parcl_id, Some(params))
    .await?;
println!("Fetched {} months of data", history.items.len());
```

Works for both GET and batch POST endpoints.

## Error Handling

```rust
use parcllabs::{ParclClient, ParclError};

match client.search().markets(params).await {
    Ok(markets) => println!("Found {} markets", markets.items.len()),
    Err(ParclError::MissingApiKey) => eprintln!("Set PARCL_LABS_API_KEY"),
    Err(ParclError::RateLimited { attempts, message }) => {
        eprintln!("Rate limited after {} retries: {}", attempts, message)
    }
    Err(ParclError::ApiError { status, message }) => {
        eprintln!("API error {}: {}", status, message)
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Examples

```bash
export PARCL_LABS_API_KEY=your_api_key

cargo run --example search_markets           # Find markets by name/location
cargo run --example market_metrics           # Housing prices, sales, inventory
cargo run --example investor_activity        # Investor buy/sell trends
cargo run --example institutional_ownership  # Ownership rate analysis
cargo run --example for_sale_market_analysis # Inventory & price drops
cargo run --example batch_metrics            # Multi-market batch queries
```

## API Coverage

53 endpoints across 9 services

| Service | GET Endpoints | Batch POST |
|---------|--------------|------------|
| Search | `markets` | — |
| Market Metrics | `housing_event_counts`, `housing_event_prices`, `housing_stock`, `all_cash`, `housing_event_property_attributes` | All 5 |
| Investor Metrics | `housing_stock_ownership`, `purchase_to_sale_ratio`, `housing_event_counts`, `housing_event_prices`, `new_listings_for_sale_rolling_counts` | All 5 |
| For-Sale Metrics | `for_sale_inventory`, `for_sale_inventory_price_changes`, `new_listings_rolling_counts` | All 3 |
| Rental Metrics | `gross_yield`, `rental_units_concentration`, `new_listings_for_rent_rolling_counts` | All 3 |
| Price Feed | `history`, `rental_history` | Both |
| New Construction | `housing_event_counts`, `housing_event_prices` | Both |
| Portfolio Metrics | `sf_housing_stock_ownership`, `sf_housing_event_counts`, `sf_new_listings_for_sale_rolling_counts`, `sf_new_listings_for_rent_rolling_counts` | All 4 |
| Property | `search` (GET), `search_by_address` (POST), `event_history` (POST), `search_v2` (POST) | — |

## License

MIT
