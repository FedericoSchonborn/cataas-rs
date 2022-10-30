#![warn(clippy::pedantic, clippy::cargo)]

use anyhow::Result;
use cataas::Client;
use clap::Parser;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Get,
    Say,
    /// List available tags
    Tags,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut user_agent = USER_AGENT.to_owned();
    user_agent.push(' ');
    user_agent.push_str(cataas::USER_AGENT);
    let client = Client::builder().user_agent(user_agent).build();

    match args.command {
        Command::Get | Command::Say => {
            todo!()
        }
        Command::Tags => client
            .tags()
            .await?
            .iter()
            .filter(|tag| !tag.is_empty())
            .for_each(|tag| println!("{}", tag)),
    }

    println!("Hello, world!");

    Ok(())
}
