use std::path::PathBuf;

use clap::{Args, Subcommand, Parser};
use derive::ExternalDerive;

#[derive(Args)]
pub struct ListArgs {
    dir: PathBuf,
    limit: u64
}

#[derive(ExternalDerive, Subcommand)]
pub enum Commands {
    #[id(b"LS")]
    List(
        #[id(b"")]
        ListArgs
    )
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    Cli::parse();
}
