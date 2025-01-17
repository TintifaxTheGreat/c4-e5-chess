extern crate c4_e5_chess;

use c4_e5_chess::engine::game::Game;
use log::LevelFilter;
use serial_test::serial;
use test_case::test_case;

const LOG_LOCATION: &str = "c4e5r.log";

#[cfg(test)]
#[ctor::ctor]
fn init() {
    simple_logging::log_to_file(LOG_LOCATION, LevelFilter::Error)
        .expect("Logfile cannot be opened.");
}
static TEST_RESOURCE: &[&str] = &[
    "2b3rk/1q3p1p/p1p1pPpQ/4N3/2pP4/2P1p1P1/1P4PK/5R2 w - - 1 1,f1h1",
    "r1b2k1r/pppq3p/2np1p2/8/2B2B2/8/PPP3PP/4RR1K w - - 0 1,f4h6",
    "1rb4r/pkPp3p/1b1P3n/1Q6/N3Pp2/8/P1P3PP/7K w - - 0 1,b5d5",
    "4r1k1/5bpp/2p5/3pr3/8/1B3pPq/PPR2P2/2R2QK1 b - - 0 1,e5e1",
    "R6R/1r3pp1/4p1kp/3pP3/1r2qPP1/7P/1P1Q3K/8 w - - 0 1,f4f5",
];

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[serial]
fn test_mate_position(i: usize) {
    let str: Vec<&str> = TEST_RESOURCE[i].split(",").collect();
    let mut g = Game::new(str[0].to_string(), 0, 15000);
    log::info!("Test case: {}", i);
    if let Some(m) = g.find_move() {
        assert_eq!(m.to_string(), str[1]);
    }
}
