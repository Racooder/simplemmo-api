use simplemmo_api::Client;

// Usage:
// SIMPLEMMO_API_KEY=<your_api_key> GUILD_ID=<guild_id> cargo run --example guild_members

#[tokio::main]
async fn main() -> simplemmo_api::Result<()> {
    let key = std::env::var("SIMPLEMMO_API_KEY").expect("set SIMPLEMMO_API_KEY");
    let guild_id: u64 = std::env::var("GUILD_ID")
        .expect("set GUILD_ID")
        .parse()
        .unwrap();
    let client = Client::builder().api_key(key).build()?;

    println!("{:#?}", client.guild().members(guild_id).await?);
    Ok(())
}
