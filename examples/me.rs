use simplemmo_api::Client;

#[tokio::main]
async fn main() -> simplemmo_api::Result<()> {
    let key = std::env::var("SIMPLEMMO_API_KEY").expect("set SIMPLEMMO_API_KEY");
    let client = Client::builder().api_key(key).build()?;

    println!("{:#?}", client.player().me().await?);
    Ok(())
}
