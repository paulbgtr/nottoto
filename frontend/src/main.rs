mod args;
mod cli;
mod requests;
mod utils;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let client = reqwest::Client::new();

    // todo: fix unwrap
    cli::handle_args(args, client).await.unwrap();
}
