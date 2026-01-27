# parcllabs-rs

Rust SDK for the [Parcl Labs API](https://docs.parcllabs.com/)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
parcllabs = { git = "https://github.com/your-username/parcllabs-rust" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use parcllabs::{ParclClient, MetricsParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client (reads PARCL_LABS_API_KEY from environment)
    let client = ParclClient::new()?;

    // Search for markets
    let markets = client
        .search()
        .markets("Los Angeles", None, None, Some(5))
        .await?;

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
# Copy the example and add your key
cp .env.example .env
```

Or export directly:

```bash
export PARCL_LABS_API_KEY=your_api_key
```

Or pass it in code:

```rust
let client = ParclClient::with_api_key("your_api_key");
```

## Endpoints

### Search

Find markets by name, state, or location type:

```rust
use parcllabs::LocationType;

// Search by query
let markets = client.search().markets("Miami", None, None, None).await?;

// Search by state and type
let ca_cities = client
    .search()
    .markets_by_state("CA", Some(LocationType::City), Some(10))
    .await?;
```

### Market Metrics

Get housing data for a specific market:

```rust
let parcl_id = 2899845; //LA

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

// Housing prices
let prices = client
    .market_metrics()
    .housing_event_prices(parcl_id, None)
    .await?;
```

### Price Feed

Get price feed data for tradeable markets:

```rust
let feed = client
    .price_feed()
    .history(parcl_id, Some(MetricsParams::new().limit(30)))
    .await?;
```

## Query Parameters

Use `MetricsParams` for pagination and date filtering:

```rust
let params = MetricsParams::new()
    .limit(100)
    .offset(0)
    .start_date("2024-01-01")
    .end_date("2024-12-31");
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

## To Do

- Add CI/CD Coverage

## License

MIT
