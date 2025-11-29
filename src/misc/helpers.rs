use cozy_chess::{util, Board};
use std::str::FromStr;

/// Parse EPD (Extended Position Description)
/// EPD parsing is not comprehensive, but it is enough to be used in some tests.
#[allow(dead_code)]
pub fn parse_epd(epd: String) -> (String, Vec<String>, bool) {
    // remove everything from the string after the first semicolon
    let epd = epd.split(';').next().expect("Cannot parse EPD.");
    let find_best_move: bool = epd.contains("bm");
    let delimiter = if find_best_move { " bm " } else { " am " };
    let mut s = epd.split(delimiter);
    let fen = s.next().expect("Cannot parse EPD.");
    let fen = format!("{fen} 0 1");
    let board = Board::from_str(&fen).expect("Invalid board.");

    let expected_moves_s = s.next().expect("Cannot parse EPD.");
    let expected_moved: Vec<&str> = expected_moves_s.split(' ').collect();
    let mut valid_best_moves = Vec::new();

    for best_move in expected_moved {
        let best_move = best_move.trim();
        let mv = util::parse_san_move(&board, best_move).expect("Invalid move.");
        valid_best_moves.push(mv.to_string());
    }
    (fen.to_string(), valid_best_moves, find_best_move)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_epd() {
        let epd =
            "r4rk1/pp2qpp1/2p1bn1p/8/1bP4Q/5N1P/PPB2PP1/R1BR2K1 w - - bm Bxh6; id \"11.IQ.1263\";";
        let (fen, expected_moves, find_best_move) = parse_epd(epd.to_string());
        assert_eq!(
            fen,
            "r4rk1/pp2qpp1/2p1bn1p/8/1bP4Q/5N1P/PPB2PP1/R1BR2K1 w - - 0 1"
        );
        assert_eq!(expected_moves, vec!["c1h6".to_string()]);
        assert_eq!(find_best_move, true);

        let epd = "r4rk1/pp2qpp1/2p1bn1p/8/1bP4Q/5N1P/PPB2PP1/R1BR2K1 w - - bm Bxh6";
        let (fen, expected_moves, find_best_move) = parse_epd(epd.to_string());
        assert_eq!(
            fen,
            "r4rk1/pp2qpp1/2p1bn1p/8/1bP4Q/5N1P/PPB2PP1/R1BR2K1 w - - 0 1"
        );
        assert_eq!(expected_moves, vec!["c1h6".to_string()]);
        assert_eq!(find_best_move, true);

        let epd = "r1b2rk1/ppp3p1/4p2p/4Qpq1/3P4/2PB4/PPK2PPP/R6R b - - am Qxg2";
        let (fen, expected_moves, find_best_move) = parse_epd(epd.to_string());
        assert_eq!(
            fen,
            "r1b2rk1/ppp3p1/4p2p/4Qpq1/3P4/2PB4/PPK2PPP/R6R b - - 0 1"
        );
        assert_eq!(expected_moves, vec!["g5g2".to_string()]);
        assert_eq!(find_best_move, false);

        let epd = "r1b2k1r/1p4pp/p4B2/2bpN3/8/q2n4/P1P2PPP/1R1QR1K1 w - - bm Bxg7+ Qh5; id \"5.IQ.1244\";";
        let (fen, expected_moves, find_best_move) = parse_epd(epd.to_string());
        assert_eq!(
            fen,
            "r1b2k1r/1p4pp/p4B2/2bpN3/8/q2n4/P1P2PPP/1R1QR1K1 w - - 0 1"
        );
        assert!(
            expected_moves == vec!["f6g7".to_string(), "d1h5".to_string()]
                || expected_moves == vec!["d1h5".to_string(), "f6g7".to_string()]
        );
        assert_eq!(find_best_move, true);
    }
}
