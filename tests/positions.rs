extern crate c4_e5_chess;

use c4_e5_chess::engine::game::Game;
use log::LevelFilter;
use serial_test::serial;
use test_case::test_case;

const LOG_LOCATION: &str = "c4e5chess.log";

#[cfg(test)]
#[ctor::ctor]
fn init() {
    simple_logging::log_to_file(LOG_LOCATION, LevelFilter::Error)
        .expect("Logfile cannot be opened.");
}
static TEST_RESOURCE: &[&str] = &[
    "8/2Q5/8/6q1/2K5/8/8/7k b - - 0 1,g5c1",
    "8/8/8/8/2R5/3k4/5K1n/8 w - - 0 1,c4h4",
    //"3q1rk1/4bp1p/1n2P2Q/1p1p1p2/6r1/Pp2R2N/1B1P2PP/7K w - - 0 1,h3g5",
];

#[test_case(0)]
#[test_case(1)]
#[serial]
fn test_position(i: usize) {
    let str: Vec<&str> = TEST_RESOURCE[i].split(",").collect();
    let mut g = Game::new(str[0].to_string(), 0, 15000);
    if let Some(m) = g.find_move() {
        assert_eq!(m.to_string(), str[1]);
    }
}
