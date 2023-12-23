mod args;
mod cli;
mod requests;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let client = reqwest::Client::new();

    cli::handle_args(args, client).await.unwrap();
}
