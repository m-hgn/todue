use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// The markdown file to use
    #[clap(default_value_t = String::from("TODO.md"))]
    file: String,

    /// Whether to ignore deadlines
    #[arg(long)]
    ignore_deadlines: bool,

    /// Whether to ignore priorities
    #[arg(long)]
    ignore_priorities: bool,
}

