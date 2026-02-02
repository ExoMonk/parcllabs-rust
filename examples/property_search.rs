//! Property API examples: search, address lookup, and event history.
//!
//! Usage: cargo run --example property_search

use parcllabs::{
    AddressSearchRequest, EventHistoryParams, EventType, ParclClient, PropertySearchParams,
    PropertyType, PropertyV2SearchRequest, PropertyFilters, V2EventFilters,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    // --- 1. Property search (GET /v1/property/search) ---
    println!("=== Property Search (Chicago, Single Family) ===\n");

    let params = PropertySearchParams::new(5387853, PropertyType::SingleFamily)
        .limit(3)
        .bedrooms_min(3)
        .event_history_sale_flag(true);

    let results = client.property().search(params).await?;

    println!("Found properties (showing first {}):", results.items.len());
    for prop in &results.items {
        println!(
            "  {} — {} bed/{} bath, {} sqft ({})",
            prop.address.as_deref().unwrap_or("N/A"),
            prop.bedrooms.unwrap_or(0),
            prop.bathrooms.unwrap_or(0.0),
            prop.square_footage.unwrap_or(0),
            prop.zip_code.as_deref().unwrap_or("N/A"),
        );
    }

    if let Some(acct) = &results.account {
        println!(
            "\nCredits used: {}, remaining: {}",
            acct.est_credits_used.unwrap_or(0),
            acct.est_remaining_credits.unwrap_or(0),
        );
    }

    // Grab a property ID for event history
    let property_id = results
        .items
        .first()
        .map(|p| p.parcl_property_id)
        .unwrap_or(63325076);

    // --- 2. Address search (POST /v1/property/search_address) ---
    println!("\n=== Address Search ===\n");

    let addresses = vec![AddressSearchRequest {
        address: "1225 W SCHOOL ST".into(),
        city: "CHICAGO".into(),
        state_abbreviation: "IL".into(),
        zip_code: "60657".into(),
    }];

    match client.property().search_by_address(addresses).await {
        Ok(resp) => {
            for item in &resp.items {
                println!(
                    "  {} → parcl_property_id: {}",
                    item.address.as_deref().unwrap_or("N/A"),
                    item.parcl_property_id,
                );
            }
        }
        Err(e) => println!("  Address search error: {}", e),
    }

    // --- 3. Event history (POST /v1/property/event_history) ---
    println!("\n=== Event History (property_id: {}) ===\n", property_id);

    let params = EventHistoryParams::new(vec![property_id]).event_type(EventType::All);

    match client.property().event_history(params).await {
        Ok(resp) => {
            for prop in &resp.properties {
                if let Some(events) = &prop.events {
                    println!("  Events for property {}:", prop.parcl_property_id);
                    for event in events {
                        println!(
                            "    {} {} — ${} ({})",
                            event.event_date.as_deref().unwrap_or("N/A"),
                            event.event_name.as_deref().unwrap_or("N/A"),
                            event.price.unwrap_or(0),
                            event.event_type.as_deref().unwrap_or("N/A"),
                        );
                    }
                }
            }
        }
        Err(e) => println!("  Event history error: {}", e),
    }

    // --- 4. V2 property search (POST /v2/property_search) ---
    println!("\n=== V2 Property Search (Chicago, 4+ bed SF) ===\n");

    let request = PropertyV2SearchRequest {
        parcl_ids: Some(vec![5387853]),
        property_filters: Some(PropertyFilters {
            include_property_details: Some(true),
            property_types: Some(vec!["SINGLE_FAMILY".into()]),
            min_beds: Some(4),
            ..Default::default()
        }),
        event_filters: Some(V2EventFilters {
            include_events: Some(true),
            event_names: Some(vec!["SOLD".into()]),
            ..Default::default()
        }),
        ..Default::default()
    };

    match client.property().search_v2(request, Some(3), None).await {
        Ok(resp) => {
            println!("Found {} properties:", resp.properties.len());
            for prop in &resp.properties {
                if let Some(meta) = &prop.property_metadata {
                    println!(
                        "  {} — {} bed, {} sqft, built {}",
                        meta.address1.as_deref().unwrap_or("N/A"),
                        meta.bedrooms.unwrap_or(0),
                        meta.sq_ft.unwrap_or(0),
                        meta.year_built.unwrap_or(0),
                    );
                }
                if let Some(events) = &prop.events {
                    for event in events.iter().take(2) {
                        println!(
                            "    {} {} — ${}",
                            event.event_date.as_deref().unwrap_or("N/A"),
                            event.event_name.as_deref().unwrap_or("N/A"),
                            event.price.unwrap_or(0),
                        );
                    }
                }
            }
        }
        Err(e) => println!("  V2 search error: {}", e),
    }

    Ok(())
}
