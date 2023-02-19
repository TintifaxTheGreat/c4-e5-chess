use crate::cmd::cli::Cli;

mod cmd;
mod engine;

fn main() {
    let mut cli = Cli::new();
    cli.execute();
}