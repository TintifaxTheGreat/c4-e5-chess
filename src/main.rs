use crate::cmd::cli::Cli;

mod cmd;
mod engine;

fn main() {
    let cli = Cli{};
    cli.execute();
}