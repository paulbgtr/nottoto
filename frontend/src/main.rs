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
    let is_logged_in = auth::is_logged_in(&client).await?;

    let _ = cli::handle_args(is_logged_in, args, client).await?;

    Ok(())
}
