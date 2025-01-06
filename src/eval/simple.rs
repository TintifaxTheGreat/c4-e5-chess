use super::{
    constants::*,
    evaluation::Evaluation,
    helpers::{
        defending_kings_moves_count, half_open_files, kings_distance, multiple_on_file, open_files,
    },
};
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

        let white_figures = b.colors(Color::White);
        let black_figures = b.colors(Color::Black);

        let white_pawns = b.pieces(Piece::Pawn) & white_figures;
        let black_pawns = b.pieces(Piece::Pawn) & black_figures;
        let white_knights = b.pieces(Piece::Knight) & white_figures;
        let black_knights = b.pieces(Piece::Knight) & black_figures;
        let white_bishops = b.pieces(Piece::Bishop) & white_figures;
        let black_bishops = b.pieces(Piece::Bishop) & black_figures;
        let white_rooks = b.pieces(Piece::Rook) & white_figures;
        let black_rooks = b.pieces(Piece::Rook) & black_figures;
        let white_queens = b.pieces(Piece::Queen) & white_figures;
        let black_queens = b.pieces(Piece::Queen) & black_figures;
        let white_king = b.pieces(Piece::King) & white_figures;
        let black_king = b.pieces(Piece::King) & black_figures;

        // Rules concerning pawns
        value += (white_pawns.0.count_ones() * 200) as MoveScore;
        value -= (black_pawns.0.count_ones() * 200) as MoveScore;

        value += ((white_pawns.0 & CB_CENTER_0_GOOD).count_ones() * 20) as MoveScore;
        value -= ((black_pawns.0 & CB_CENTER_0_GOOD).count_ones() * 20) as MoveScore;

        value += ((white_pawns.0 & CB_CENTER_1).count_ones() * 30) as MoveScore;
        value -= ((black_pawns.0 & CB_CENTER_1).count_ones() * 30) as MoveScore;

        value += ((white_pawns.0 & CB_RANK_6).count_ones() * 250) as MoveScore;
        value -= ((black_pawns.0 & CB_RANK_3).count_ones() * 250) as MoveScore;

        value += ((white_pawns.0 & CB_RANK_7).count_ones() * 650) as MoveScore;
        value -= ((black_pawns.0 & CB_RANK_2).count_ones() * 650) as MoveScore;

        value -= (multiple_on_file(white_pawns.0) * 30) as MoveScore;
        value += (multiple_on_file(black_pawns.0) * 30) as MoveScore;

        // Rules concerning knights
        value += (white_knights.0.count_ones() * 600) as MoveScore;
        value -= (black_knights.0.count_ones() * 600) as MoveScore;

        value -= ((white_knights.0 & CB_BOARD_0).count_ones() * 29) as MoveScore;
        value += ((black_knights.0 & CB_BOARD_0).count_ones() * 29) as MoveScore;

        // Rules concerning bishops
        value += (white_bishops.0.count_ones() * 620) as MoveScore;
        value -= (black_bishops.0.count_ones() * 620) as MoveScore;

        // Rules concerning rooks
        value += (white_rooks.0.count_ones() * 950) as MoveScore;
        value -= (black_rooks.0.count_ones() * 950) as MoveScore;

        value += ((white_rooks.0 & b_open_files).count_ones() * 20) as MoveScore;
        value -= ((black_rooks.0 & b_open_files).count_ones() * 20) as MoveScore;

        value += ((white_rooks.0 & b_half_open_files).count_ones() * 10) as MoveScore;
        value -= ((black_rooks.0 & b_half_open_files).count_ones() * 10) as MoveScore;

        value += ((white_rooks.0 & CB_RANK_7).count_ones() * 100) as MoveScore;
        value -= ((black_rooks.0 & CB_RANK_2).count_ones() * 100) as MoveScore;

        // Rules concerning queens
        value += (white_queens.0.count_ones() * 1800) as MoveScore;
        value -= (black_queens.0.count_ones() * 1800) as MoveScore;

        value -= ((white_queens.0 & CB_CENTER).count_ones() * 30) as MoveScore;
        value += ((black_queens.0 & CB_CENTER).count_ones() * 30) as MoveScore;

        if pieces_count > 20 {
            value -= ((white_knights.0 & CB_RANK_1).count_ones() * 51) as MoveScore;
            value += ((black_knights.0 & CB_RANK_8).count_ones() * 51) as MoveScore;

            value -= ((white_bishops.0 & CB_RANK_1).count_ones() * 51) as MoveScore;
            value += ((black_bishops.0 & CB_RANK_8).count_ones() * 51) as MoveScore;

            value += ((white_bishops.0 & CB_GOOD_BISHOP).count_ones() * 20) as MoveScore;
            value -= ((black_bishops.0 & CB_GOOD_BISHOP).count_ones() * 20) as MoveScore;

            value += ((white_queens.0 & CB_GOOD_QUEEN).count_ones() * 30) as MoveScore;
            value -= ((black_queens.0 & CB_GOOD_QUEEN).count_ones() * 30) as MoveScore;

            value += ((white_king.0 & CB_SAFE_KING).count_ones() * 150) as MoveScore;
            value -= ((black_king.0 & CB_SAFE_KING).count_ones() * 150) as MoveScore;
        }

        let defending_king: u64;
        if b.side_to_move() == Color::White {
            defending_king = white_king.0;
        } else {
            defending_king = black_king.0;
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
