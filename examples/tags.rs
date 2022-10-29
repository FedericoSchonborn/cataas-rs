use anyhow::Result;
use cataas::Client;
use clap::Parser;

/// List all tags available in CATAAS
#[derive(Debug, Parser)]
struct Args;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let _ = Args::parse();

    let client = Client::new();
    println!("{}", client.tags().await?.join(", "));

    Ok(())
}
