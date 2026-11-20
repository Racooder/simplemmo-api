use simplemmo_api::Client;

// Usage:
// SIMPLEMMO_API_KEY=<your_api_key> PLAYER_ID=<player_id> cargo run --example player_equipment

#[tokio::main]
async fn main() -> simplemmo_api::Result<()> {
    let key = std::env::var("SIMPLEMMO_API_KEY").expect("set SIMPLEMMO_API_KEY");
    let player_id: u64 = std::env::var("PLAYER_ID")
        .expect("set PLAYER_ID")
        .parse()
        .unwrap();
    let client = Client::builder().api_key(key).build()?;

    println!("{:#?}", client.player().equipment(player_id).await?);
    Ok(())
}
