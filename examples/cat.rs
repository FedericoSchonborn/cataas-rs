use std::env;

use anyhow::Result;
use cataas::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let gif = env::args().nth(1).is_some();

    let client = Client::new();
    let mut request = client.cat();
    if gif {
        request.gif();
    }
    let cat = request.send().await?;

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
