use simplemmo_api::Client;

// Usage:
// SIMPLEMMO_API_KEY=<your_api_key> cargo run --example me

#[tokio::main]
async fn main() -> simplemmo_api::Result<()> {
    let key = std::env::var("SIMPLEMMO_API_KEY").expect("set SIMPLEMMO_API_KEY");
    let client = Client::builder().api_key(key).build()?;

    println!("{:#?}", client.player().me().await?);
    Ok(())
}
