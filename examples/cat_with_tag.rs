use std::env;

use anyhow::{Context, Result};
use cataas::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let tag = env::args().nth(1).context("missing tag argument")?;

    let client = Client::new();
    let cat = client.cat().tag(tag).send().await?;

    println!("        ID: {}", cat.id);
    println!("      File: {}", cat.file);
    println!("       URL: https://cataas.com{}", cat.url);
    println!(
        "      Size: {}",
        cat.size
            .map(|size| size.to_string())
            .as_deref()
            .unwrap_or("Unknown")
    );
    println!("     Owner: {}", cat.owner);
    println!("Created At: {}", cat.created_at);
    println!("Updated At: {}", cat.updated_at);
    println!(
        "      Tags: {}",
        cat.tags
            .map(|tags| {
                tags.iter()
                    .map(|tag| format!("#{}", tag))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .as_deref()
            .unwrap_or("None")
    );

    Ok(())
}
