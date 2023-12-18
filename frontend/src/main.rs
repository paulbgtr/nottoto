mod args;
mod cli;
mod requests;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
}
