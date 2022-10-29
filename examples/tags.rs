use anyhow::Result;
use cataas::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let client = Client::new();
    println!("{}", client.tags().await?.join(", "));

    Ok(())
}
