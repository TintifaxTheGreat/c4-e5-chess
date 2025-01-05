use super::constants::CB_RANK_1;
use crate::misc::types::*;
use cozy_chess::{Board, Color, Move, Piece};
use std::cmp::max;

/// Gives the number of available moves for the defending king.
pub fn defending_kings_moves_count(b: &Board) -> usize {
    match b.null_move() {
        Some(b1) => {
            let kings_square = b1.king_square(b1.side_to_move());
            let kings_moves: Vec<Move> = MoveGen::new_legal(&b1)
                .filter(|m| m.get_source() == kings_square)
                .collect();
            kings_moves.len()
        }
        None => 0,
    }
}

/// Calculate the distance between both kings.
pub fn kings_distance(b: &Board) -> MoveScore {
    let wk = b.king_square(Color::White);
    let bk = b.king_square(Color::Black);
    max(
        MoveScore::abs(wk.get_rank() as MoveScore - bk.get_rank() as MoveScore),
        MoveScore::abs(wk.get_file() as MoveScore - bk.get_file() as MoveScore),
    )
}

/// Fill the bitboard to the north of the set field.
pub fn north_fill(p: u64) -> u64 {
    let mut result = p | (p << 8);
    result = result | (result << 16);
    result | (result << 32)
}

/// Fill the bitboard to the south of the set field.
pub fn south_fill(p: u64) -> u64 {
    let mut result = p | (p >> 8);
    result = result | (result >> 16);
    result | (result >> 32)
}

/// Fill the file of the set field.
pub fn file_fill(p: u64) -> u64 {
    north_fill(p) | south_fill(p)
}

/// Give all open files (files without pawns).
pub fn open_files(b: &Board) -> u64 {
    let pawns = b.pieces(Piece::Pawn).0;
    !file_fill(pawns)
}

/// Give all half open files (files with own pawn, but no oppisite pawn).
pub fn half_open_files(b: &Board) -> u64 {
    let white = file_fill(b.pieces(Piece::Pawn).0 & b.color_combined(Color::White).0);
    let black = file_fill(b.pieces(Piece::Pawn).0 & b.color_combined(Color::Black).0);
    (white & !black) | (!white & black)
}

/// Give all double or multiple pawns.
pub fn multiple_on_file(pp: u64) -> u32 {
    pp.count_ones() - (file_fill(pp) & CB_RANK_1).count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_defending_kings_moves_count() -> Result<(), chess::Error> {
        let board = Board::from_str("8/7k/8/5r2/1KN5/2R5/8/8 w - - 0 1")?;
        assert_eq!(defending_kings_moves_count(&board), 5);

        let board = Board::from_str("8/7k/8/5r2/1KN5/2R5/8/8 b - - 0 1")?;
        assert_eq!(defending_kings_moves_count(&board), 3);

        let board = Board::from_str("8/3K4/8/3k4/8/5N2/8/2Q1R3 w - - 0 1")?;
        assert_eq!(defending_kings_moves_count(&board), 0);

        Ok(())
    }

    #[test]
    fn test_kings_distance() -> Result<(), chess::Error> {
        let board = Board::from_str("8/7k/8/5r2/1KN5/2R5/8/8 w - - 0 1")?;
        assert_eq!(kings_distance(&board), 6);
        Ok(())
    }

    #[test]
    fn test_multiple_on_file() {
        assert_eq!(multiple_on_file(8659230720), 1);
        assert_eq!(multiple_on_file(36028805676099584), 0);
        assert_eq!(multiple_on_file(36028805735055360), 4);
    }

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
    fn test_open_files() -> Result<(), chess::Error> {
        let board = Board::from_str("rnbqkbnr/p1ppp1p1/8/8/8/8/P1P1PPP1/RNBQKBNR w KQkq - 0 1")?;
        assert_eq!(open_files(&board), 0x8282828282828282);
        Ok(())
    }

    #[test]
    fn test_half_open_files() -> Result<(), chess::Error> {
        let board = Board::from_str("rnbqkbnr/p1ppp1p1/8/8/8/8/P1P1PPP1/RNBQKBNR w KQkq - 0 1")?;
        assert_eq!(half_open_files(&board), 0x2828282828282828);
        Ok(())
    }
}
