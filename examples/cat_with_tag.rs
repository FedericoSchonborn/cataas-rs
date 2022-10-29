use anyhow::Result;
use cataas::Client;
use clap::Parser;

/// Get a random cat, searching by tag
#[derive(Debug, Parser)]
struct Args {
    /// Name of the tag
    tag: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = Client::new();
    let cat = client.cat().tag(args.tag).send().await?;

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
