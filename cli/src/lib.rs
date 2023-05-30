pub mod command;
pub mod validator;

use clap::{command, Parser};
use command::Commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
