extern crate c4_e5_chess;

use c4_e5_chess::{engine::game::Game, misc::helpers::parse_epd};
use log::LevelFilter;
use serial_test::serial;
use test_case::test_case;

const LOG_LOCATION: &str = "c4e5chess.log";

#[cfg(test)]
#[ctor::ctor]
fn init() {
    simple_logging::log_to_file(LOG_LOCATION, LevelFilter::Info)
        .expect("Logfile cannot be opened.");
}

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[serial]
fn test_debug(i: usize) {
    let test_resource = include_str!("epd/debug.epd").lines().collect::<Vec<&str>>();
    let (fen, expected_moves, find_best_move) = parse_epd(test_resource[i].to_string());
    let mut g = Game::new(fen, 0, 6000);
    if let Some(m) = g.find_move() {
        if find_best_move {
            assert!(expected_moves.contains(&m.to_string()));
        } else {
            assert!(!expected_moves.contains(&m.to_string()));
        }
    }
}
