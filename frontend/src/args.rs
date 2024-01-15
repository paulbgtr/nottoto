use clap::Parser;

#[derive(Parser, Debug)]
#[clap(group(
    clap::ArgGroup::new("command")
    .required(true)
    .args(&[
        "all",
        "create",
        "rename",
        "edit",
        "delete",
        "find",
        "view",
        "login",
        "join",
        "quit"
    ])
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

    #[arg(short, long)]
    pub view: Option<String>,

    #[arg(short, long)]
    pub login: bool,

    #[arg(short, long)]
    pub join: bool,

    #[arg(short, long)]
    pub quit: bool,
}
