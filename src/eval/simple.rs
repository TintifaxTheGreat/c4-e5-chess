use super::{constants::*, evaluation::Evaluation, helpers::*};
use crate::misc::types::*;
use cozy_chess::{Board, Color, Piece};

#[derive(Default)]
pub struct Simple {}

impl Evaluation for Simple {
    /// A simple static evaluation function for the given board position
    fn evaluate(b: &Board) -> MoveScore {
        let mut value: MoveScore = 0;
        let pieces_count = b.occupied().len();

        let b_open_files = open_files(b);
        let b_half_open_files = half_open_files(b);

        // Rules concerning pawns
        value += (b.colored_pieces(Color::White, Piece::Pawn).0.count_ones() * 200) as MoveScore;
        value -= (b.colored_pieces(Color::Black, Piece::Pawn).0.count_ones() * 200) as MoveScore;

        value += ((b.colored_pieces(Color::White, Piece::Pawn).0 & CB_CENTER_0_GOOD).count_ones()
            * 20) as MoveScore;
        value -= ((b.colored_pieces(Color::Black, Piece::Pawn).0 & CB_CENTER_0_GOOD).count_ones()
            * 20) as MoveScore;

        value += ((b.colored_pieces(Color::White, Piece::Pawn).0 & CB_CENTER_1).count_ones() * 30)
            as MoveScore;
        value -= ((b.colored_pieces(Color::Black, Piece::Pawn).0 & CB_CENTER_1).count_ones() * 30)
            as MoveScore;

        value += ((b.colored_pieces(Color::White, Piece::Pawn).0 & CB_RANK_6).count_ones() * 250)
            as MoveScore;
        value -= ((b.colored_pieces(Color::Black, Piece::Pawn).0 & CB_RANK_3).count_ones() * 250)
            as MoveScore;

        value += ((b.colored_pieces(Color::White, Piece::Pawn).0 & CB_RANK_7).count_ones() * 650)
            as MoveScore;
        value -= ((b.colored_pieces(Color::Black, Piece::Pawn).0 & CB_RANK_2).count_ones() * 650)
            as MoveScore;

        value -=
            (multiple_on_file(b.colored_pieces(Color::White, Piece::Pawn).0) * 30) as MoveScore;
        value +=
            (multiple_on_file(b.colored_pieces(Color::Black, Piece::Pawn).0) * 30) as MoveScore;

        // Rules concerning knights
        value += (b.colored_pieces(Color::White, Piece::Knight).0.count_ones() * 600) as MoveScore;
        value -= (b.colored_pieces(Color::Black, Piece::Knight).0.count_ones() * 600) as MoveScore;

        value -= ((b.colored_pieces(Color::White, Piece::Knight).0 & CB_BOARD_0).count_ones() * 29)
            as MoveScore;
        value += ((b.colored_pieces(Color::Black, Piece::Knight).0 & CB_BOARD_0).count_ones() * 29)
            as MoveScore;

        // Rules concerning bishops
        value += (b.colored_pieces(Color::White, Piece::Bishop).0.count_ones() * 620) as MoveScore;
        value -= (b.colored_pieces(Color::Black, Piece::Bishop).0.count_ones() * 620) as MoveScore;

        // Rules concerning rooks
        value += (b.colored_pieces(Color::White, Piece::Rook).0.count_ones() * 950) as MoveScore;
        value -= (b.colored_pieces(Color::Black, Piece::Rook).0.count_ones() * 950) as MoveScore;

        value += ((b.colored_pieces(Color::White, Piece::Rook).0 & b_open_files).count_ones() * 20)
            as MoveScore;
        value -= ((b.colored_pieces(Color::Black, Piece::Rook).0 & b_open_files).count_ones() * 20)
            as MoveScore;

        value += ((b.colored_pieces(Color::White, Piece::Rook).0 & b_half_open_files).count_ones()
            * 10) as MoveScore;
        value -= ((b.colored_pieces(Color::Black, Piece::Rook).0 & b_half_open_files).count_ones()
            * 10) as MoveScore;

        value += ((b.colored_pieces(Color::White, Piece::Rook).0 & CB_RANK_7).count_ones() * 100)
            as MoveScore;
        value -= ((b.colored_pieces(Color::Black, Piece::Rook).0 & CB_RANK_2).count_ones() * 100)
            as MoveScore;

        // Rules concerning queens
        value += (b.colored_pieces(Color::White, Piece::Queen).0.count_ones() * 1800) as MoveScore;
        value -= (b.colored_pieces(Color::Black, Piece::Queen).0.count_ones() * 1800) as MoveScore;

        value -= ((b.colored_pieces(Color::White, Piece::Queen).0 & CB_CENTER).count_ones() * 30)
            as MoveScore;
        value += ((b.colored_pieces(Color::Black, Piece::Queen).0 & CB_CENTER).count_ones() * 30)
            as MoveScore;

        if pieces_count > 20 {
            value -= ((b.colored_pieces(Color::White, Piece::Knight).0 & CB_RANK_1).count_ones()
                * 51) as MoveScore;
            value += ((b.colored_pieces(Color::Black, Piece::Knight).0 & CB_RANK_8).count_ones()
                * 51) as MoveScore;

            value -= ((b.colored_pieces(Color::White, Piece::Bishop).0 & CB_RANK_1).count_ones()
                * 51) as MoveScore;
            value += ((b.colored_pieces(Color::Black, Piece::Bishop).0 & CB_RANK_8).count_ones()
                * 51) as MoveScore;

            value += ((b.colored_pieces(Color::White, Piece::Bishop).0 & CB_GOOD_BISHOP)
                .count_ones()
                * 20) as MoveScore;
            value -= ((b.colored_pieces(Color::Black, Piece::Bishop).0 & CB_GOOD_BISHOP)
                .count_ones()
                * 20) as MoveScore;

            value += ((b.colored_pieces(Color::White, Piece::Queen).0 & CB_GOOD_QUEEN).count_ones()
                * 30) as MoveScore;
            value -= ((b.colored_pieces(Color::Black, Piece::Queen).0 & CB_GOOD_QUEEN).count_ones()
                * 30) as MoveScore;

            value += ((b.colored_pieces(Color::White, Piece::King).0 & CB_SAFE_KING).count_ones()
                * 150) as MoveScore;
            value -= ((b.colored_pieces(Color::Black, Piece::King).0 & CB_SAFE_KING).count_ones()
                * 150) as MoveScore;
        }

        let defending_king: u64;
        if b.side_to_move() == Color::White {
            defending_king = b.colored_pieces(Color::White, Piece::King).0;
        } else {
            defending_king = b.colored_pieces(Color::Black, Piece::King).0;
            value *= -1;
        }
        if pieces_count < 8 {
            let mut kings_value: MoveScore = kings_distance(b) * 10;
            kings_value += defending_kings_moves_count(b) as MoveScore * 10;
            kings_value += ((defending_king & CB_CENTER_0).count_ones() * 80) as MoveScore;
            kings_value += ((defending_king & CB_CENTER_1).count_ones() * 40) as MoveScore;
            kings_value += ((defending_king & CB_BOARD_1).count_ones() * 10) as MoveScore;
            kings_value -= ((defending_king & CB_BOARD_0).count_ones() * 50) as MoveScore;

            if value > 0 {
                kings_value *= -1;
            }
            value += kings_value;
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cozy_chess::Board;
    use std::str::FromStr;

    #[test]
    fn test_evaluate_initial_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_str(fen).expect("Invalid FEN");
        let score = Simple::evaluate(&board);
        assert_eq!(score, 0); // Initial position should be balanced
    }

    #[test]
    fn test_evaluate_white_advantage() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
        let board = Board::from_str(fen).expect("Invalid FEN");
        let score = Simple::evaluate(&board);
        assert!(score > 0); // White has an advantage with an extra pawn in the center
    }

    #[test]
    fn test_evaluate_black_advantage() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/4p3/PPPP1PPP/RNBQKBNR w KQkq - 0 2";
        let board = Board::from_str(fen).expect("Invalid FEN");
        let score = Simple::evaluate(&board);
        assert!(score < 0); // Black has an advantage with an extra pawn in the center
    }

    #[test]
    fn test_evaluate_endgame() {
        let fen = "8/8/8/8/8/8/4K3/4k3 w - - 0 1";
        let board = Board::from_str(fen).expect("Invalid FEN");
        let score = Simple::evaluate(&board);
        assert_eq!(score, 0); // King vs King should be balanced
    }

    #[test]
    fn test_evaluate_complex_position() {
        let fen = "r1bqkbnr/pppppppp/2n5/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 2 3";
        let board = Board::from_str(fen).expect("Invalid FEN");
        let score = Simple::evaluate(&board);
        assert!(score > 0); // White has an advantage with better development
    }
}
