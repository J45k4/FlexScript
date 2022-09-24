use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Rawast(RawastArgs)
}

#[derive(Debug, Parser)]
pub struct RawastArgs {
    pub path: String
}