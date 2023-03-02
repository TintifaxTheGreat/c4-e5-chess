use log::{info, LevelFilter};
use crate::cmd::cli::Cli;
mod cmd;
mod engine;

fn main() {
    _ = simple_logging::log_to_file("test.log", LevelFilter::Info);
    info! ("programm has started");
    let mut cli = Cli::new();
    cli.execute();
}