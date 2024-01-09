mod args;
mod auth;
mod cli;
mod requests;
mod utils;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();
    let client = reqwest::Client::new();

    let _ = cli::handle_args(args, client).await?;

    Ok(())
}
