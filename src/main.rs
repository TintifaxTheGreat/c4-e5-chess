use crate::cmd::cli::Cli;
use log::{info, LevelFilter};
mod cmd;
mod engine;
mod misc;
mod eval;

/// Main function to be used with a UCI chess gui
fn main() {
    match simple_logging::log_to_file("c4e5r.log", LevelFilter::Info) {
        Ok(_) => {
            let mut cli = Cli::new();
            info!("Startup completed.");
            cli.execute();
        }

        Err(_) => panic!("Can't open logfile."),
    }
}
