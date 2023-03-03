use crate::cmd::cli::Cli;
use log::{info, LevelFilter};
mod cmd;
mod engine;

fn main() {
    match simple_logging::log_to_file("/home/eugen/work/rust/c4e5r/test.log", LevelFilter::Info) {
        Ok(_) => {
            let mut cli = Cli::new();
            info!("Startup completed.");
            cli.execute();
        }

        Err(_) => panic!("Can't open logfile."),
    }
}
