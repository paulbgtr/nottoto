use clap::Parser;

#[derive(Parser, Debug)]
#[clap(group(
    clap::ArgGroup::new("command")
    .required(true)
    .args(&["create", "update", "delete", "find"])
    .required(true)
))]
pub struct Args {
    #[arg(short, long)]
    create: Option<String>,

    #[arg(short, long)]
    update: Option<String>,

    #[arg(short, long)]
    delete: Option<String>,

    #[arg(short, long)]
    find: Option<String>,
}
