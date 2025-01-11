use cozy_chess::{util, Board};
use std::str::FromStr;

/// Parse EPD (Extended Position Description)
#[allow(dead_code)]
pub fn parse_epd(epd: String) -> (String, Vec<String>, bool) {
    let find_best_move: bool = epd.contains("bm");
    let delimiter = if find_best_move { " bm " } else { " am " };
    let mut s = epd.split(delimiter);
    let fen = s.next().expect("Cannot parse EPD.");
    let fen = format!("{} 0 1", fen);
    let board = Board::from_str(&fen).expect("Invalid board.");

    let expected_moves_s = s.next().expect("Cannot parse EPD.");
    let expected_moved: Vec<&str> = expected_moves_s.split(',').collect();
    let mut valid_best_moves = Vec::new();

    for best_move in expected_moved {
        let best_move = best_move.trim();
        let mv = util::parse_san_move(&board, best_move).expect("Invalid move.");
        valid_best_moves.push(mv.to_string());
    }
    (fen.to_string(), valid_best_moves, find_best_move)
}
