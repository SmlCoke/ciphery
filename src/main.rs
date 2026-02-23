mod cli;
mod handler;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    handler::run(cli.command.as_ref());
}
