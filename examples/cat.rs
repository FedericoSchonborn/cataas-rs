use anyhow::Result;
use cataas::Client;
use clap::Parser;

/// Get a random cat
#[derive(Debug, Parser)]
struct Args {
    /// Get an animated cat
    #[clap(long)]
    gif: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = Client::new();
    let mut request = client.cat();
    if args.gif {
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
