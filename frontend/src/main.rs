mod args;
mod requests;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
}
