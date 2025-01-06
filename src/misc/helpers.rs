use std::str::FromStr;

use cozy_chess::{Board, Move};

/// Parse EPD (Extended Position Description)
#[allow(dead_code)]
pub fn parse_epd(epd: String) -> (String, String) {
    let mut s = epd.split(" bm ");
    let fen = s.next().expect("Cannot parse EPD.");
    let best_move = s.next().expect("Cannot parse EPD.");
    let _board = Board::from_fen(fen, true).expect("Invalid board.");
    let mv = Move::from_str(best_move).expect("Invalid move.");
    (fen.to_string(), mv.to_string())
}
