use simplemmo_api::Client;

// Usage:
// SIMPLEMMO_API_KEY=<your_api_key> GUILD_ID=<guild_id> cargo run --example guild_member_contribution

#[tokio::main]
async fn main() -> simplemmo_api::Result<()> {
    let key = std::env::var("SIMPLEMMO_API_KEY").expect("set SIMPLEMMO_API_KEY");
    let guild_id: u64 = std::env::var("GUILD_ID")
        .expect("set GUILD_ID")
        .parse()
        .unwrap();
    let player_id: u64 = std::env::var("PLAYER_ID")
        .expect("set PLAYER_ID")
        .parse()
        .unwrap();
    let client = Client::builder().api_key(key).build()?;

    println!(
        "{:#?}",
        client
            .guild()
            .member_contribution(guild_id, player_id)
            .await?
    );
    Ok(())
}
