use clap::Parser;

#[derive(Parser, Debug)]
#[clap(group(
    clap::ArgGroup::new("command")
    .required(true)
    .args(&["create", "update", "delete", "find"])
))]
pub struct Args {
    #[arg(short, long)]
    pub create: Option<String>,

    #[arg(short, long)]
    pub update: Option<String>,

    #[arg(short, long)]
    pub delete: Option<String>,

    #[arg(short, long)]
    pub find: Option<String>,
}
