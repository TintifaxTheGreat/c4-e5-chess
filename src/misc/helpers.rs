use chess::{Board, ChessMove};
use std::str::FromStr;

/// Parse EPD (Extended Position Description)
pub fn parse_epd(epd: String) -> (String, String) {
    let mut s = epd.split(" bm ");
    let fen = s.next().expect("Cannot parse EPD.");
    let best_move = s.next().expect("Cannot parse EPD.");
    let board = Board::from_str(fen).expect("Invalid board.");
    let mv = ChessMove::from_san(&board, best_move).expect("Invalid move.");
    (fen.to_string(), mv.to_string())
}
