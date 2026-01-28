//! Analyze institutional ownership rates across housing markets.
//!
//! This example demonstrates how to:
//! - Compare investor ownership percentages across markets
//! - Track ownership trends over time
//! - Identify markets with high/low institutional presence
//! - Combine ownership data with pricing data for yield analysis
//!
//! Use Case: Homebuyers researching markets with lower investor competition,
//! or investors identifying underserved markets with growth potential.
//!
//! Usage: cargo run --example institutional_ownership

use parcllabs::{InvestorMetricsParams, MetricsParams, ParclClient, PropertyType, SearchParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    println!("==========================================================");
    println!("  INSTITUTIONAL OWNERSHIP ANALYSIS");
    println!("  Finding markets with varying investor concentration");
    println!("==========================================================\n");

    // Analyze Sun Belt metros (known for high investor activity)
    let sun_belt = vec![
        ("Austin", "TX"),
        ("Phoenix", "AZ"),
        ("Jacksonville", "FL"),
        ("Nashville", "TN"),
        ("Raleigh", "NC"),
        ("San Antonio", "TX"),
        ("Orlando", "FL"),
        ("Denver", "CO"),
    ];

    println!("Analyzing {} Sun Belt metros...\n", sun_belt.len());

    #[derive(Debug)]
    struct MarketOwnership {
        name: String,
        parcl_id: i64,
        investor_pct: f64,
        investor_count: i64,
        median_price: f64,
        price_per_sqft: f64,
    }

    let mut ownership_data: Vec<MarketOwnership> = Vec::new();

    for (city, state) in &sun_belt {
        let params = SearchParams::new().query(*city).state(*state).limit(1);

        let markets = client.search().markets(params).await?;

        if let Some(market) = markets.items.first() {
            // Get ownership data
            let ownership = client
                .investor_metrics()
                .housing_stock_ownership(
                    market.parcl_id,
                    Some(InvestorMetricsParams::new().limit(1)),
                )
                .await?;

            // Get pricing data for single family homes
            let prices = client
                .market_metrics()
                .housing_event_prices(
                    market.parcl_id,
                    Some(
                        MetricsParams::new()
                            .limit(1)
                            .property_type(PropertyType::SingleFamily),
                    ),
                )
                .await?;

            if let Some(own) = ownership.items.first() {
                let mut median_price = 0.0;
                let mut price_per_sqft = 0.0;

                if let Some(price_item) = prices.items.first() {
                    if let Some(ref price) = price_item.price {
                        if let Some(ref median) = price.median {
                            median_price = median.sales.unwrap_or(0.0);
                        }
                    }
                    if let Some(ref ppsf) = price_item.price_per_square_foot {
                        if let Some(ref median) = ppsf.median {
                            price_per_sqft = median.sales.unwrap_or(0.0);
                        }
                    }
                }

                ownership_data.push(MarketOwnership {
                    name: format!("{}, {}", city, state),
                    parcl_id: market.parcl_id,
                    investor_pct: own.investor_owned_pct.unwrap_or(0.0),
                    investor_count: own.investor_owned_count.unwrap_or(0),
                    median_price,
                    price_per_sqft,
                });
            }
        }
    }

    // Sort by investor ownership percentage
    ownership_data.sort_by(|a, b| {
        b.investor_pct
            .partial_cmp(&a.investor_pct)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Display ownership comparison
    println!("INVESTOR OWNERSHIP BY METRO (Highest to Lowest):\n");
    println!(
        "{:<18} {:>12} {:>14} {:>14} {:>10}",
        "Metro", "Investor %", "Inv. Units", "Median Price", "$/SqFt"
    );
    println!("{}", "-".repeat(72));

    for data in &ownership_data {
        let intensity = if data.investor_pct > 15.0 {
            "🔴 High"
        } else if data.investor_pct > 10.0 {
            "🟡 Medium"
        } else {
            "🟢 Low"
        };

        println!(
            "{:<18} {:>10.1}% {:>14} {:>13}k {:>9}",
            data.name,
            data.investor_pct,
            format_number(data.investor_count),
            format!("${:.0}", data.median_price / 1000.0),
            format!("${:.0}", data.price_per_sqft)
        );
        println!("{:<18} {}", "", intensity);
    }

    // Identify extremes
    if let (Some(highest), Some(lowest)) = (ownership_data.first(), ownership_data.last()) {
        println!("\n==========================================================");
        println!("  KEY INSIGHTS");
        println!("==========================================================\n");

        println!(
            "📈 HIGHEST Investor Concentration: {} ({:.1}%)",
            highest.name, highest.investor_pct
        );
        println!(
            "   - {} investor-owned units",
            format_number(highest.investor_count)
        );
        println!("   - Higher competition for buyers, potential rental market strength\n");

        println!(
            "📉 LOWEST Investor Concentration: {} ({:.1}%)",
            lowest.name, lowest.investor_pct
        );
        println!(
            "   - {} investor-owned units",
            format_number(lowest.investor_count)
        );
        println!("   - Less competition, potential opportunity for investors\n");

        let avg_pct: f64 = ownership_data.iter().map(|d| d.investor_pct).sum::<f64>()
            / ownership_data.len() as f64;
        println!(
            "📊 Average Investor Ownership Across Markets: {:.1}%",
            avg_pct
        );
    }

    // Deep dive into a specific market
    if let Some(focus_market) = ownership_data.first() {
        println!("\n==========================================================");
        println!(
            "  TREND ANALYSIS: {} (parcl_id: {})",
            focus_market.name, focus_market.parcl_id
        );
        println!("==========================================================\n");

        // Get 12-month ownership trend
        let historical = client
            .investor_metrics()
            .housing_stock_ownership(
                focus_market.parcl_id,
                Some(InvestorMetricsParams::new().limit(12).auto_paginate(true)),
            )
            .await?;

        println!("12-MONTH INVESTOR OWNERSHIP TREND:\n");
        println!(
            "{:<12} {:>12} {:>14}  Trend",
            "Date", "Ownership %", "Units"
        );
        println!("{}", "-".repeat(55));

        let mut prev_pct = 0.0;
        for (i, item) in historical.items.iter().rev().enumerate() {
            let pct = item.investor_owned_pct.unwrap_or(0.0);
            let count = item.investor_owned_count.unwrap_or(0);

            let trend = if i == 0 {
                "—"
            } else if pct > prev_pct + 0.1 {
                "↑"
            } else if pct < prev_pct - 0.1 {
                "↓"
            } else {
                "→"
            };

            // Visual bar
            let bar_len = (pct * 2.0).min(40.0) as usize;
            let bar = "▓".repeat(bar_len);

            println!(
                "{:<12} {:>10.1}% {:>14}  {} {}",
                item.date,
                pct,
                format_number(count),
                trend,
                bar
            );

            prev_pct = pct;
        }

        // Calculate change
        if let (Some(oldest), Some(newest)) = (historical.items.last(), historical.items.first()) {
            let old_pct = oldest.investor_owned_pct.unwrap_or(0.0);
            let new_pct = newest.investor_owned_pct.unwrap_or(0.0);
            let change = new_pct - old_pct;

            println!("\n12-Month Change: {:+.2}%", change);
            if change > 0.5 {
                println!("📈 Investors are INCREASING their presence in this market");
            } else if change < -0.5 {
                println!("📉 Investors are REDUCING their presence in this market");
            } else {
                println!("➡️  Investor presence is relatively STABLE");
            }
        }
    }

    // Compare investor activity metrics
    println!("\n==========================================================");
    println!("  INVESTOR TRANSACTION ACTIVITY (Latest Month)");
    println!("==========================================================\n");

    println!(
        "{:<18} {:>12} {:>12} {:>12}",
        "Metro", "Acquisitions", "Dispositions", "Net Activity"
    );
    println!("{}", "-".repeat(58));

    for data in ownership_data.iter().take(5) {
        let events = client
            .investor_metrics()
            .housing_event_counts(data.parcl_id, Some(InvestorMetricsParams::new().limit(1)))
            .await?;

        if let Some(ev) = events.items.first() {
            let acq = ev.acquisitions.unwrap_or(0);
            let disp = ev.dispositions.unwrap_or(0);
            let net = acq - disp;

            let net_str = if net > 0 {
                format!("+{} 📈", net)
            } else if net < 0 {
                format!("{} 📉", net)
            } else {
                "0 ➡️".to_string()
            };

            println!(
                "{:<18} {:>12} {:>12} {:>12}",
                data.name,
                format_number(acq),
                format_number(disp),
                net_str
            );
        }
    }

    println!("\n==========================================================");
    println!("  ANALYSIS COMPLETE");
    println!("==========================================================\n");

    println!("💡 TIP: Markets with lower investor ownership may offer:");
    println!("   - Less competition for homebuyers");
    println!("   - Potential entry points for investors\n");

    println!("💡 TIP: Markets with higher investor ownership may indicate:");
    println!("   - Strong rental demand");
    println!("   - Institutional confidence in appreciation");

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
