use super::{
    constants::*,
    helpers::{defending_kings_moves_count, half_open_files, kings_distance, open_files},
    types::*,
};
use chess::{Board, Color, Piece};

pub fn evaluate(b: &Board) -> MoveScore {
    let mut value: MoveScore = 0;
    let pieces_count = b.combined().count();

    let b_open_files = open_files(b);
    let b_half_open_files = half_open_files(b);

    let white_figures = b.color_combined(Color::White).0;
    let black_figures = b.color_combined(Color::Black).0;

    let white_pawns = b.pieces(Piece::Pawn).0 & white_figures;
    let black_pawns = b.pieces(Piece::Pawn).0 & black_figures;
    let white_knights = b.pieces(Piece::Knight).0 & white_figures;
    let black_knights = b.pieces(Piece::Knight).0 & black_figures;
    let white_bishops = b.pieces(Piece::Bishop).0 & white_figures;
    let black_bishops = b.pieces(Piece::Bishop).0 & black_figures;
    let white_rooks = b.pieces(Piece::Rook).0 & white_figures;
    let black_rooks = b.pieces(Piece::Rook).0 & black_figures;
    let white_queens = b.pieces(Piece::Queen).0 & white_figures;
    let black_queens = b.pieces(Piece::Queen).0 & black_figures;
    let white_king = b.pieces(Piece::King).0 & white_figures;
    let black_king = b.pieces(Piece::King).0 & black_figures;

    // Rules concerning pawns
    value += (white_pawns.count_ones() * 200) as MoveScore;
    value -= (black_pawns.count_ones() * 200) as MoveScore;

    value += ((white_pawns & CB_CENTER_0).count_ones() * 40) as MoveScore;
    value -= ((black_pawns & CB_CENTER_0).count_ones() * 40) as MoveScore;

    value += ((white_pawns & CB_7TH_RANK).count_ones() * 300) as MoveScore;
    value -= ((black_pawns & CB_2ND_RANK).count_ones() * 300) as MoveScore;

    // Rules concerning knights
    value += (white_knights.count_ones() * 600) as MoveScore;
    value -= (black_knights.count_ones() * 600) as MoveScore;

    value -= ((white_knights & CB_BOARD_0).count_ones() * 20) as MoveScore;
    value += ((black_knights & CB_BOARD_0).count_ones() * 20) as MoveScore;

    // Rules concerning bishops
    value += (white_bishops.count_ones() * 620) as MoveScore;
    value -= (black_bishops.count_ones() * 620) as MoveScore;

    // Rules concerning rooks
    value += (white_rooks.count_ones() * 950) as MoveScore;
    value -= (black_rooks.count_ones() * 950) as MoveScore;

    value += ((white_rooks & b_open_files).count_ones() * 20) as MoveScore;
    value -= ((black_rooks & b_open_files).count_ones() * 20) as MoveScore;

    value += ((white_rooks & b_half_open_files).count_ones() * 10) as MoveScore;
    value -= ((black_rooks & b_half_open_files).count_ones() * 10) as MoveScore;

    // Rules concerning queens
    value += (white_queens.count_ones() * 1800) as MoveScore;
    value -= (black_queens.count_ones() * 1800) as MoveScore;

    value -= ((white_queens & CB_CENTER).count_ones() * 30) as MoveScore;
    value += ((black_queens & CB_CENTER).count_ones() * 30) as MoveScore;

    if pieces_count > 20 {
        value -= ((white_knights & CB_BASE_LINE).count_ones() * 40) as MoveScore;
        value += ((black_knights & CB_BASE_LINE).count_ones() * 40) as MoveScore;

        value -= ((white_bishops & CB_BASE_LINE).count_ones() * 40) as MoveScore;
        value += ((black_bishops & CB_BASE_LINE).count_ones() * 40) as MoveScore;

        value += ((white_bishops & CB_GOOD_BISHOP).count_ones() * 20) as MoveScore;
        value -= ((black_bishops & CB_GOOD_BISHOP).count_ones() * 20) as MoveScore;

        value += ((white_queens & CB_GOOD_QUEEN).count_ones() * 30) as MoveScore;
        value -= ((black_queens & CB_GOOD_QUEEN).count_ones() * 30) as MoveScore;

        value += ((white_king & CB_SAFE_KING).count_ones() * 130) as MoveScore;
        value -= ((black_king & CB_SAFE_KING).count_ones() * 130) as MoveScore;
    }

    let defending_king: u64;
    if b.side_to_move() == Color::White {
        defending_king = white_king;
    } else {
        defending_king = black_king;
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
    return value;
}
