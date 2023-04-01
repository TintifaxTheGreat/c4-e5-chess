extern crate c4e5r;

use c4e5r::engine::game::Game;
use log::LevelFilter;
use test_case::test_case;

const LOG_LOCATION: &str = "/home/eugen/work/rust/c4e5r/test.log";

#[cfg(test)]
#[ctor::ctor]
fn init() {
    simple_logging::log_to_file(LOG_LOCATION, LevelFilter::Info)
        .expect("Logfile cannot be opened.");
}
static TEST_RESOURCE: &[&str] = &[
    "2b3rk/1q3p1p/p1p1pPpQ/4N3/2pP4/2P1p1P1/1P4PK/5R2 w - - 1 1,f1h1",
    "r1b2k1r/pppq3p/2np1p2/8/2B2B2/8/PPP3PP/4RR1K w - - 0 1,f4h6",
    "1rb4r/pkPp3p/1b1P3n/1Q6/N3Pp2/8/P1P3PP/7K w - - 1 1,b5d5",
    "8/2Q5/8/6q1/2K5/8/8/7k b - - 0 1,g5c1",
    "8/8/8/8/2R5/3k4/5K1n/8 w - - 0 1,c4h4",
    "4r1k1/5bpp/2p5/3pr3/8/1B3pPq/PPR2P2/2R2QK1 b - - 0 1,e5e1",
    "3q1rk1/4bp1p/1n2P2Q/1p1p1p2/6r1/Pp2R2N/1B1P2PP/7K w - - 1 0,h3g5",
];

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
#[test_case(6)]
fn test_position(i: usize) {
    let str: Vec<&str> = TEST_RESOURCE[i].split(",").collect();
    let mut g = Game::new(str[0].to_string(), 99, 5000);
    if let Some(m) = g.find_move() {
        assert_eq!(m.to_string(), str[1]);
    }
}
