use cozy_chess::{Board, Move};
use std::str::FromStr;

/// Parse EPD (Extended Position Description)
#[allow(dead_code)]
pub fn parse_epd(epd: String) -> (String, String) {
    let mut s = epd.split(" bm ");
    let fen = s.next().expect("Cannot parse EPD.");
    let fen = format!("{} 0 1", fen);

    let best_move = s.next().expect("Cannot parse EPD.");
    let _board = Board::from_str(&fen).expect("Invalid board.");
    let mv = Move::from_str(best_move).expect("Invalid move.");
    (fen.to_string(), mv.to_string())
}
