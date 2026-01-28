# parcllabs-rust

[![CI](https://github.com/ExoMonk/parcllabs-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/ExoMonk/parcllabs-rust/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/parcllabs.svg)](https://crates.io/crates/parcllabs)
[![Documentation](https://docs.rs/parcllabs/badge.svg)](https://docs.rs/parcllabs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.83.0-blue.svg)](https://blog.rust-lang.org/2024/11/28/Rust-1.83.0.html)

Rust SDK for the [Parcl Labs API](https://docs.parcllabs.com/) - Real-time U.S. housing market data and analytics covering 70,000+ markets.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
parcllabs = "0.1"
tokio = { version = "1", features = ["full"] }
```

Or use `cargo add`:

```bash
cargo add parcllabs
cargo add tokio --features full
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

Get your API key at [dashboard.parcllabs.com](https://dashboard.parcllabs.com).

```bash
# Set environment variable
export PARCL_LABS_API_KEY=your_api_key
```

Or pass it directly:

```rust
let client = ParclClient::with_api_key("your_api_key");

// Custom base URL (for testing)
let client = ParclClient::with_config("your_api_key", "https://custom.api.com");
```

---

## Services

### Search Markets

Find markets by name, state, location type, or region. Returns `parcl_id` identifiers used by all other endpoints.

```rust
use parcllabs::{SearchParams, LocationType, USRegion, SortBy, SortOrder};

let params = SearchParams::new()
    .query("San Francisco")         // Search by name
    .state("CA")                    // Filter by state
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

    // Check if market has price feed data
    if market.has_price_feed() {
        println!("  -> Price feed available");
    }
}
```

**Available Filters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `query` | `String` | Search text (city name, ZIP, etc.) |
| `state` | `String` | State abbreviation (CA, NY, TX...) |
| `location_type` | `LocationType` | City, County, Zip5, Cbsa, Town, Village, Cdp, All |
| `region` | `USRegion` | Pacific, Mountain, NewEngland, MiddleAtlantic, etc. |
| `sort_by` | `SortBy` | TotalPopulation, MedianIncome, PricefeedMarket, etc. |
| `sort_order` | `SortOrder` | Asc, Desc |

---

### Market Metrics

Housing market analytics including sales counts, prices, inventory, and property attributes.

```rust
use parcllabs::{MetricsParams, PropertyType};

let parcl_id = 2900078; // Los Angeles CBSA

// Housing event counts (sales, new listings)
let events = client.market_metrics()
    .housing_event_counts(parcl_id, None)
    .await?;

// Housing stock by property type
let stock = client.market_metrics()
    .housing_stock(parcl_id, None)
    .await?;

// Prices filtered by property type and date range
let params = MetricsParams::new()
    .property_type(PropertyType::SingleFamily)
    .start_date("2024-01-01")
    .end_date("2024-12-31");

let prices = client.market_metrics()
    .housing_event_prices(parcl_id, Some(params))
    .await?;

// All-cash transaction metrics
let all_cash = client.market_metrics()
    .all_cash(parcl_id, None)
    .await?;

for item in &all_cash.items {
    println!("{}: {:.1}% all-cash sales",
        item.date,
        item.pct_sales.unwrap_or(0.0)
    );
}

// Property attributes (beds, baths, sqft, year built)
let attrs = client.market_metrics()
    .housing_event_property_attributes(parcl_id, None)
    .await?;
```

**Available Endpoints:**
| Method | Description |
|--------|-------------|
| `housing_event_counts()` | Monthly sales and listing counts |
| `housing_event_prices()` | Median prices with percentiles |
| `housing_stock()` | Property counts by type |
| `all_cash()` | All-cash transaction rates |
| `housing_event_property_attributes()` | Median beds, baths, sqft, year built |

**MetricsParams Options:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `start_date` | `String` | Filter from date (YYYY-MM-DD) |
| `end_date` | `String` | Filter to date (YYYY-MM-DD) |
| `property_type` | `PropertyType` | SingleFamily, Condo, Townhouse, Other, AllProperties |
| `limit` | `u32` | Results per page |
| `offset` | `u32` | Pagination offset |
| `auto_paginate` | `bool` | Fetch all pages automatically |

---

### Investor Metrics

Track institutional investor activity and ownership patterns.

```rust
// Investor housing stock ownership
let ownership = client.investor_metrics()
    .housing_stock_ownership(parcl_id, None)
    .await?;

// Investor purchase-to-sale ratio
let ratio = client.investor_metrics()
    .purchase_to_sale_ratio(parcl_id, None)
    .await?;

// Investor housing event counts
let events = client.investor_metrics()
    .housing_event_counts(parcl_id, None)
    .await?;

// Investor housing event prices
let prices = client.investor_metrics()
    .housing_event_prices(parcl_id, None)
    .await?;

// New listings by investors (supports property_type filter)
let params = MetricsParams::new()
    .property_type(PropertyType::SingleFamily);
let listings = client.investor_metrics()
    .new_listings_for_sale_rolling_counts(parcl_id, Some(params))
    .await?;
```

**Available Endpoints:**
| Method | Description |
|--------|-------------|
| `housing_stock_ownership()` | Investor ownership rates |
| `purchase_to_sale_ratio()` | Buy vs sell activity |
| `housing_event_counts()` | Investor transaction counts |
| `housing_event_prices()` | Investor transaction prices |
| `new_listings_for_sale_rolling_counts()` | Rolling listing counts |

---

### For-Sale Market Metrics

Track current inventory and listing activity in the for-sale market.

```rust
use parcllabs::{ForSaleMetricsParams, PropertyType};

let parcl_id = 2900078; // Los Angeles CBSA

// Current for-sale inventory count
let inventory = client.for_sale_metrics()
    .for_sale_inventory(parcl_id, None)
    .await?;

for item in &inventory.items {
    println!("{}: {} properties for sale",
        item.date,
        item.for_sale_inventory.unwrap_or(0)
    );
}

// Price change metrics (drops, days between changes)
let params = ForSaleMetricsParams::new()
    .property_type(PropertyType::SingleFamily)
    .start_date("2024-01-01");

let price_changes = client.for_sale_metrics()
    .for_sale_inventory_price_changes(parcl_id, Some(params))
    .await?;

for item in &price_changes.items {
    println!("{}: {:.1}% had price drops",
        item.date,
        item.pct_price_drop.unwrap_or(0.0)
    );
}

// Rolling counts of new listings (7, 30, 60, 90 day)
let rolling = client.for_sale_metrics()
    .new_listings_rolling_counts(parcl_id, None)
    .await?;

for item in &rolling.items {
    println!("{}: {} new listings (30-day)",
        item.date,
        item.rolling_30_day_count.unwrap_or(0)
    );
}
```

**Available Endpoints:**
| Method | Description |
|--------|-------------|
| `for_sale_inventory()` | Current inventory count |
| `for_sale_inventory_price_changes()` | Price change metrics |
| `new_listings_rolling_counts()` | Rolling counts (7/30/60/90 day) |

**ForSaleMetricsParams Options:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `start_date` | `String` | Filter from date (YYYY-MM-DD) |
| `end_date` | `String` | Filter to date (YYYY-MM-DD) |
| `property_type` | `PropertyType` | SingleFamily, Condo, Townhouse, Other, AllProperties |
| `limit` | `u32` | Results per page |
| `offset` | `u32` | Pagination offset |
| `auto_paginate` | `bool` | Fetch all pages automatically |

---

### Price Feed

Historical price indices for tradeable markets.

```rust
let feed = client.price_feed()
    .history(parcl_id, Some(MetricsParams::new().limit(30)))
    .await?;

for entry in &feed.items {
    println!("{}: ${:.2}", entry.date, entry.price);
}
```

---

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

// Metrics - fetch complete history
let params = MetricsParams::new()
    .start_date("2020-01-01")
    .auto_paginate(true);

let history = client.market_metrics()
    .housing_event_counts(parcl_id, Some(params))
    .await?;
println!("Fetched {} months of data", history.items.len());
```

---

## Examples

```bash
# Set up environment
export PARCL_LABS_API_KEY=your_api_key

# Run examples
cargo run --example search_markets           # Find markets by name/location
cargo run --example market_metrics           # Housing prices, sales, inventory
cargo run --example investor_activity        # Track investor buy/sell trends
cargo run --example institutional_ownership  # Analyze ownership rates
cargo run --example for_sale_market_analysis # Inventory & price drop analysis
```

### Example: Investor Activity Analysis

Track where institutional investors are buying or selling:

```rust
use parcllabs::{ParclClient, InvestorMetricsParams};

let client = ParclClient::new()?;

// Get purchase-to-sale ratio (>1 = net buyer, <1 = net seller)
let ratios = client.investor_metrics()
    .purchase_to_sale_ratio(parcl_id, None)
    .await?;

for item in &ratios.items {
    let ratio = item.purchase_to_sale_ratio.unwrap_or(0.0);
    let status = if ratio > 1.0 { "Net Buyer" } else { "Net Seller" };
    println!("{}: {:.2} ({})", item.date, ratio, status);
}

// Get rolling listing counts
let listings = client.investor_metrics()
    .new_listings_for_sale_rolling_counts(parcl_id, None)
    .await?;
```

### Example: Institutional Ownership

Compare investor ownership across markets:

```rust
// Get ownership percentage
let ownership = client.investor_metrics()
    .housing_stock_ownership(parcl_id, None)
    .await?;

if let Some(latest) = ownership.items.first() {
    println!("Investor owned: {:.1}% ({} units)",
        latest.investor_owned_pct.unwrap_or(0.0),
        latest.investor_owned_count.unwrap_or(0)
    );
}

// Track investor transactions
let events = client.investor_metrics()
    .housing_event_counts(parcl_id, None)
    .await?;

if let Some(ev) = events.items.first() {
    let net = ev.acquisitions.unwrap_or(0) - ev.dispositions.unwrap_or(0);
    println!("Net activity: {} properties", net);
}
```

---

## API Coverage

### Implemented

| Service | Endpoint | Method |
|---------|----------|--------|
| **Search** | `/v1/search/markets` | `search().markets()` |
| **Market Metrics** | `housing_event_counts` | `market_metrics().housing_event_counts()` |
| **Market Metrics** | `housing_event_prices` | `market_metrics().housing_event_prices()` |
| **Market Metrics** | `housing_stock` | `market_metrics().housing_stock()` |
| **Market Metrics** | `all_cash` | `market_metrics().all_cash()` |
| **Market Metrics** | `housing_event_property_attributes` | `market_metrics().housing_event_property_attributes()` |
| **Investor Metrics** | `housing_stock_ownership` | `investor_metrics().housing_stock_ownership()` |
| **Investor Metrics** | `purchase_to_sale_ratio` | `investor_metrics().purchase_to_sale_ratio()` |
| **Investor Metrics** | `housing_event_counts` | `investor_metrics().housing_event_counts()` |
| **Investor Metrics** | `housing_event_prices` | `investor_metrics().housing_event_prices()` |
| **Investor Metrics** | `new_listings_for_sale_rolling_counts` | `investor_metrics().new_listings_for_sale_rolling_counts()` |
| **For-Sale Metrics** | `for_sale_inventory` | `for_sale_metrics().for_sale_inventory()` |
| **For-Sale Metrics** | `for_sale_inventory_price_changes` | `for_sale_metrics().for_sale_inventory_price_changes()` |
| **For-Sale Metrics** | `new_listings_rolling_counts` | `for_sale_metrics().new_listings_rolling_counts()` |
| **Price Feed** | `history` | `price_feed().history()` |

### Planned

| Service | Endpoints | Priority |
|---------|-----------|----------|
| **Rental Metrics** | gross_yield, rental_units_concentration, rolling_counts | High |
| **New Construction** | housing_event_counts, housing_event_prices | Medium |
| **Portfolio Metrics** | sf_housing_stock_ownership, sf_event_counts, rolling_counts | Medium |
| **Price Feed** | rental_price_feed | Medium |
| **Property API** | property_search, event_history, address_search | Low |
| **Batch Endpoints** | POST endpoints for bulk queries | Low |

---

## Error Handling

```rust
use parcllabs::{ParclClient, ParclError};

match client.search().markets(params).await {
    Ok(markets) => println!("Found {} markets", markets.items.len()),
    Err(ParclError::MissingApiKey) => eprintln!("Set PARCL_LABS_API_KEY"),
    Err(ParclError::ApiError { status, message }) => {
        eprintln!("API error {}: {}", status, message)
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## License

MIT
