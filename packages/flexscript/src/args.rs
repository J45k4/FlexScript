use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Rawast(RawastArgs),
    Ast(AstArgs),
}

#[derive(Debug, Parser)]
pub struct RawastArgs {
    pub path: String
}

#[derive(Debug, Parser)]
pub struct AstArgs {
    pub path: String
}