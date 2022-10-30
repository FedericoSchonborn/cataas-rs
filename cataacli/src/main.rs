#![warn(clippy::pedantic, clippy::cargo)]
#![doc = include_str!("../README.md")]

use anyhow::{bail, Result};
use cataas::{
    types::{Filter, ImageType},
    Client,
};
use clap::Parser;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Get(GetArgs),
    Say(SayArgs),
    /// List available tags
    Tags,
}

#[derive(Debug, Parser)]
struct GetArgs {
    #[clap(long)]
    tag: Option<String>,
    #[clap(long)]
    gif: bool,
    #[clap(long = "type", id = "TYPE")]
    image_type: Option<ImageType>,
    #[clap(long)]
    filter: Option<Filter>,
    #[clap(long)]
    width: Option<usize>,
    #[clap(long)]
    height: Option<usize>,
}
#[derive(Debug, Parser)]
struct SayArgs {
    text: String,
    #[clap(long)]
    tag: Option<String>,
    #[clap(long)]
    gif: bool,
    #[clap(long = "type", id = "TYPE")]
    image_type: Option<ImageType>,
    #[clap(long)]
    filter: Option<Filter>,
    #[clap(long)]
    width: Option<usize>,
    #[clap(long)]
    height: Option<usize>,
    #[clap(long)]
    size: Option<usize>,
    #[clap(long)]
    color: Option<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut user_agent = USER_AGENT.to_owned();
    user_agent.push(' ');
    user_agent.push_str(cataas::client::USER_AGENT);
    let client = Client::builder().user_agent(user_agent).build();

    match args.command {
        Command::Get(args) => get(&client, args).await,
        Command::Say(args) => say(&client, args).await,
        Command::Tags => tags(&client).await,
    }
}

async fn get(client: &Client, args: GetArgs) -> Result<()> {
    if args.tag.is_some() && args.gif {
        bail!("cannot use `--tag` with `--gif`");
    }

    let cat = client
        .cat()
        .with_tag(args.tag)
        .with_gif(args.gif)
        .with_image_type(args.image_type)
        .with_filter(args.filter)
        .with_width(args.width)
        .with_height(args.height)
        .send()
        .await?;
    println!("https://cataas.com{}", cat.url);
    Ok(())
}

async fn say(client: &Client, args: SayArgs) -> Result<()> {
    if args.tag.is_some() && args.gif {
        bail!("cannot use `--tag` with `--gif`");
    }

    let cat = client
        .says(args.text)
        .with_tag(args.tag)
        .with_gif(args.gif)
        .with_image_type(args.image_type)
        .with_filter(args.filter)
        .with_width(args.width)
        .with_height(args.height)
        .with_size(args.size)
        .with_color(args.color)
        .send()
        .await?;
    println!("https://cataas.com{}", cat.url);
    Ok(())
}

async fn tags(client: &Client) -> Result<()> {
    client
        .tags()
        .await?
        .iter()
        .filter(|tag| !tag.is_empty())
        .for_each(|tag| println!("{}", tag));

    Ok(())
}
