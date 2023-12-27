use clap::Parser;

#[derive(Parser, Debug)]
#[clap(group(
    clap::ArgGroup::new("command")
    .required(true)
    .args(&["all", "create", "rename" , "edit", "delete", "find"])
))]
pub struct Args {
    #[arg(short, long)]
    pub all: bool,

    #[arg(short, long)]
    pub create: Option<String>,

    #[arg(short, long)]
    pub edit: Option<String>,

    #[arg(short, long)]
    pub rename: Option<String>,

    #[arg(short, long)]
    pub delete: Option<String>,

    #[arg(short, long)]
    pub find: Option<String>,
}
