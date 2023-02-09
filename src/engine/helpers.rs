use chess::{Board, Color, Piece};

pub fn north_fill(p: u64) -> u64 {
    let mut result = p | (p << 8);
    result = result | (result << 16);
    return result | (result << 32);
}

pub fn south_fill(p: u64) -> u64 {
    let mut result = p | (p >> 8);
    result = result | (result >> 16);
    return result | (result >> 32);
}

pub fn file_fill(p: u64) -> u64 {
    return north_fill(p) | south_fill(p);
}

pub fn open_files(b: Board) -> u64 {
    let pawns = b.pieces(Piece::Pawn).0;
    return !file_fill(pawns);
}

pub fn half_open_files(b: Board) -> u64 {
    let white = file_fill(b.pieces(Piece::Pawn).0 & b.color_combined(Color::White).0);
    let black = file_fill(b.pieces(Piece::Pawn).0 & b.color_combined(Color::Black).0);
    return (white & !black) | (!white & black);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_north_fill() {
        assert_eq!(north_fill(17179869184), 289360691284934656);
    }

    #[test]
    fn test_south_fill() {
        assert_eq!(south_fill(17179869184), 17247241220);
    }
    #[test]
    fn test_file_fill() {
        assert_eq!(file_fill(17179869184), 289360691352306692);
    }

    #[test]
    fn test_open_files() {
        let board = Board::from_fen(String::from(
            "rnbqkbnr/p1ppp1p1/8/8/8/8/P1P1PPP1/RNBQKBNR w KQkq - 0 1",
        ));
        if board.is_some() {
            assert_eq!(open_files(board.unwrap()), 0x8282828282828282);
        }
    }

    #[test]
    fn test_half_open_files() {
        let board = Board::from_fen(String::from(
            "rnbqkbnr/p1ppp1p1/8/8/8/8/P1P1PPP1/RNBQKBNR w KQkq - 0 1",
        ));
        if board.is_some() {
            assert_eq!(half_open_files(board.unwrap()), 0x2828282828282828);
        }
    }
}
