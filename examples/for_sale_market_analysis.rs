//! Analyze for-sale market conditions across housing markets.
//!
//! This example demonstrates how to:
//! - Track current for-sale inventory levels
//! - Monitor price change behavior (price drops, days between changes)
//! - Analyze new listing activity with rolling counts
//! - Compare market conditions across major metros
//!
//! Use Case: Real estate agents identifying buyer's vs seller's markets,
//! or investors finding markets with motivated sellers (high price drop rates).
//!
//! Usage: cargo run --example for_sale_market_analysis

use parcllabs::{ForSaleMetricsParams, ParclClient, PropertyType, SearchParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    println!("==========================================================");
    println!("  FOR-SALE MARKET ANALYSIS");
    println!("  Inventory, Price Changes & Listing Activity");
    println!("==========================================================\n");

    // Analyze diverse markets (hot markets, cooling markets, stable markets)
    let metros = vec![
        ("Austin", "TX"),
        ("Phoenix", "AZ"),
        ("Miami", "FL"),
        ("Seattle", "WA"),
        ("Denver", "CO"),
        ("Chicago", "IL"),
    ];

    println!("Analyzing {} markets...\n", metros.len());

    #[derive(Debug)]
    struct MarketSnapshot {
        name: String,
        parcl_id: i64,
        inventory: i64,
        pct_price_drop: f64,
        median_days_bt_change: f64,
        rolling_30_day_listings: i64,
        rolling_90_day_listings: i64,
    }

    let mut snapshots: Vec<MarketSnapshot> = Vec::new();

    for (city, state) in &metros {
        let params = SearchParams::new().query(*city).state(*state).limit(1);
        let markets = client.search().markets(params).await?;

        if let Some(market) = markets.items.first() {
            // Get current inventory
            let inventory = client
                .for_sale_metrics()
                .for_sale_inventory(
                    market.parcl_id,
                    Some(
                        ForSaleMetricsParams::new()
                            .property_type(PropertyType::SingleFamily)
                            .limit(1),
                    ),
                )
                .await?;

            // Get price change metrics
            let price_changes = client
                .for_sale_metrics()
                .for_sale_inventory_price_changes(
                    market.parcl_id,
                    Some(
                        ForSaleMetricsParams::new()
                            .property_type(PropertyType::SingleFamily)
                            .limit(1),
                    ),
                )
                .await?;

            // Get rolling listing counts
            let rolling = client
                .for_sale_metrics()
                .new_listings_rolling_counts(
                    market.parcl_id,
                    Some(
                        ForSaleMetricsParams::new()
                            .property_type(PropertyType::SingleFamily)
                            .limit(1),
                    ),
                )
                .await?;

            if let (Some(inv), Some(pc), Some(roll)) = (
                inventory.items.first(),
                price_changes.items.first(),
                rolling.items.first(),
            ) {
                snapshots.push(MarketSnapshot {
                    name: format!("{}, {}", city, state),
                    parcl_id: market.parcl_id,
                    inventory: inv.for_sale_inventory.unwrap_or(0),
                    pct_price_drop: pc.pct_price_drop.unwrap_or(0.0),
                    median_days_bt_change: pc.median_days_bt_price_change.unwrap_or(0.0),
                    rolling_30_day_listings: roll.rolling_30_day_count.unwrap_or(0),
                    rolling_90_day_listings: roll.rolling_90_day_count.unwrap_or(0),
                });
            }
        }
    }

    // Sort by price drop percentage (highest first = most motivated sellers)
    snapshots.sort_by(|a, b| {
        b.pct_price_drop
            .partial_cmp(&a.pct_price_drop)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Display market comparison
    println!("MARKET CONDITIONS OVERVIEW (Single Family Homes)\n");
    println!(
        "{:<16} {:>12} {:>12} {:>14} {:>10} {:>10}",
        "Metro", "Inventory", "Price Drops", "Days Bt Chg", "30-Day", "90-Day"
    );
    println!("{}", "-".repeat(78));

    for snap in &snapshots {
        let market_type = if snap.pct_price_drop > 15.0 {
            "Buyer's Mkt"
        } else if snap.pct_price_drop < 8.0 {
            "Seller's Mkt"
        } else {
            "Balanced"
        };

        println!(
            "{:<16} {:>12} {:>10.1}% {:>14.0} {:>10} {:>10}",
            snap.name,
            format_number(snap.inventory),
            snap.pct_price_drop,
            snap.median_days_bt_change,
            format_number(snap.rolling_30_day_listings),
            format_number(snap.rolling_90_day_listings)
        );
        println!("{:<16} {}", "", market_type);
    }

    // Find extremes for insights
    println!("\n==========================================================");
    println!("  KEY INSIGHTS");
    println!("==========================================================\n");

    if let Some(highest_drops) = snapshots.first() {
        println!(
            "MOST MOTIVATED SELLERS: {} ({:.1}% with price drops)",
            highest_drops.name, highest_drops.pct_price_drop
        );
        println!("  -> Higher negotiating power for buyers");
        println!(
            "  -> Avg {:.0} days between price changes\n",
            highest_drops.median_days_bt_change
        );
    }

    if let Some(lowest_drops) = snapshots.last() {
        println!(
            "STRONGEST SELLER'S MARKET: {} ({:.1}% with price drops)",
            lowest_drops.name, lowest_drops.pct_price_drop
        );
        println!("  -> Sellers holding firm on prices");
        println!("  -> Competitive market for buyers\n");
    }

    // Deep dive into top market
    if let Some(focus) = snapshots.first() {
        println!("==========================================================");
        println!(
            "  TREND ANALYSIS: {} (parcl_id: {})",
            focus.name, focus.parcl_id
        );
        println!("==========================================================\n");

        // Get historical price change data
        let historical = client
            .for_sale_metrics()
            .for_sale_inventory_price_changes(
                focus.parcl_id,
                Some(
                    ForSaleMetricsParams::new()
                        .property_type(PropertyType::SingleFamily)
                        .limit(12),
                ),
            )
            .await?;

        println!("PRICE DROP TRENDS (Last 12 Weeks):\n");
        println!(
            "{:<12} {:>12} {:>12} {:>14}",
            "Date", "% Drops", "Count Drops", "Median Change"
        );
        println!("{}", "-".repeat(55));

        for item in historical.items.iter().rev() {
            let pct = item.pct_price_drop.unwrap_or(0.0);
            let count = item.count_price_drop.unwrap_or(0);
            let median_change = item.median_price_change.unwrap_or(0.0);

            // Visual indicator
            let bar_len = (pct * 2.0).min(30.0) as usize;
            let bar = "|".to_string() + &"#".repeat(bar_len);

            println!(
                "{:<12} {:>10.1}% {:>12} {:>13} {}",
                item.date,
                pct,
                format_number(count),
                format_currency(median_change),
                bar
            );
        }

        // Get rolling counts trend
        let rolling_history = client
            .for_sale_metrics()
            .new_listings_rolling_counts(
                focus.parcl_id,
                Some(
                    ForSaleMetricsParams::new()
                        .property_type(PropertyType::SingleFamily)
                        .limit(8),
                ),
            )
            .await?;

        println!("\nNEW LISTING VELOCITY:\n");
        println!(
            "{:<12} {:>10} {:>10} {:>10} {:>10}",
            "Date", "7-Day", "30-Day", "60-Day", "90-Day"
        );
        println!("{}", "-".repeat(55));

        for item in rolling_history.items.iter().rev() {
            println!(
                "{:<12} {:>10} {:>10} {:>10} {:>10}",
                item.date,
                format_number(item.rolling_7_day_count.unwrap_or(0)),
                format_number(item.rolling_30_day_count.unwrap_or(0)),
                format_number(item.rolling_60_day_count.unwrap_or(0)),
                format_number(item.rolling_90_day_count.unwrap_or(0))
            );
        }

        // Calculate momentum
        if rolling_history.items.len() >= 2 {
            let newest = &rolling_history.items[0];
            let oldest = &rolling_history.items[rolling_history.items.len() - 1];

            let new_30 = newest.rolling_30_day_count.unwrap_or(0) as f64;
            let old_30 = oldest.rolling_30_day_count.unwrap_or(1) as f64;
            let change_pct = ((new_30 - old_30) / old_30) * 100.0;

            println!("\n30-Day Listing Momentum: {:+.1}%", change_pct);
            if change_pct > 10.0 {
                println!("-> Supply INCREASING - shifting toward buyer's market");
            } else if change_pct < -10.0 {
                println!("-> Supply DECREASING - shifting toward seller's market");
            } else {
                println!("-> Supply STABLE");
            }
        }
    }

    println!("\n==========================================================");
    println!("  ANALYSIS COMPLETE");
    println!("==========================================================\n");

    println!("INTERPRETATION GUIDE:");
    println!("  High % Price Drops (>15%) = Buyer's market, room for negotiation");
    println!("  Low % Price Drops (<8%)   = Seller's market, competitive bidding");
    println!("  Rising 30-Day Listings    = Increasing supply, cooling market");
    println!("  Falling 30-Day Listings   = Decreasing supply, heating market");

    Ok(())
}

fn format_number(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn format_currency(n: f64) -> String {
    if n.abs() >= 1_000.0 {
        format!("${:.0}K", n / 1_000.0)
    } else {
        format!("${:.0}", n)
    }
}
