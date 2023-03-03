use super::{
    constants::*,
    helpers::{half_open_files, open_files},
};
use chess::{Board, Color, Piece};

pub fn evaluate(b: &Board, moves_count: i32) -> i32 {
    let mut value: i32 = 0;
    let pieces_count = b.combined().count();

    let b_open_files = open_files(b);
    let b_half_open_files = half_open_files(b);

    let white_pawns = b.pieces(Piece::Pawn).0 & b.color_combined(Color::White).0;
    let black_pawns = b.pieces(Piece::Pawn).0 & b.color_combined(Color::Black).0;
    let white_knights = b.pieces(Piece::Knight).0 & b.color_combined(Color::White).0;
    let black_knights = b.pieces(Piece::Knight).0 & b.color_combined(Color::Black).0;
    let white_bishops = b.pieces(Piece::Bishop).0 & b.color_combined(Color::White).0;
    let black_bishops = b.pieces(Piece::Bishop).0 & b.color_combined(Color::Black).0;
    let white_rooks = b.pieces(Piece::Rook).0 & b.color_combined(Color::White).0;
    let black_rooks = b.pieces(Piece::Rook).0 & b.color_combined(Color::Black).0;
    let white_queens = b.pieces(Piece::Queen).0 & b.color_combined(Color::White).0;
    let black_queens = b.pieces(Piece::Queen).0 & b.color_combined(Color::Black).0;
    let white_king = b.pieces(Piece::King).0 & b.color_combined(Color::White).0;
    let black_king = b.pieces(Piece::King).0 & b.color_combined(Color::Black).0;

    // Rules concerning pawns
    value += (white_pawns.count_ones() * 190) as i32;
    value -= (black_pawns.count_ones() * 200) as i32;

    value += ((white_pawns & CB_CENTER_0).count_ones() * 40) as i32;
    value -= ((black_pawns & CB_CENTER_0).count_ones() * 40) as i32;

    // Rules concerning knights
    value += (white_knights.count_ones() * 590) as i32;
    value -= (black_knights.count_ones() * 600) as i32;

    value += ((white_knights & CB_BOARD_0).count_ones() * 20) as i32;
    value -= ((black_knights & CB_BOARD_0).count_ones() * 20) as i32;

    // Rules concerning bishops
    value += (white_bishops.count_ones() * 610) as i32;
    value -= (black_bishops.count_ones() * 620) as i32;

    // Rules concerning rooks
    value += (white_rooks.count_ones() * 940) as i32;
    value -= (black_rooks.count_ones() * 950) as i32;

    value += ((white_rooks & b_open_files).count_ones() * 20) as i32;
    value -= ((black_rooks & b_open_files).count_ones() * 20) as i32;

    value += ((white_rooks & b_half_open_files).count_ones() * 10) as i32;
    value -= ((black_rooks & b_half_open_files).count_ones() * 10) as i32;

    // Rules concerning queens
    value += (white_queens.count_ones() * 1790) as i32;
    value -= (black_queens.count_ones() * 1800) as i32;

    if moves_count < 12 {
        value += ((white_queens & CB_GOOD_QUEEN).count_ones() * 120) as i32;
        value -= ((black_queens & CB_GOOD_QUEEN).count_ones() * 120) as i32;
    }

    if pieces_count > 20 {
        value -= ((white_knights & CB_BASE_LINE).count_ones() * 30) as i32;
        value += ((black_knights & CB_BASE_LINE).count_ones() * 30) as i32;

        value -= ((white_bishops & CB_BASE_LINE).count_ones() * 40) as i32;
        value += ((black_bishops & CB_BASE_LINE).count_ones() * 40) as i32;

        value += ((white_bishops & CB_GOOD_BISHOP).count_ones() * 20) as i32;
        value -= ((black_bishops & CB_GOOD_BISHOP).count_ones() * 20) as i32;

        value -= ((white_queens & CB_CENTER).count_ones() * 30) as i32;
        value += ((black_queens & CB_CENTER).count_ones() * 30) as i32;

        value += ((white_king & CB_SAFE_KING).count_ones() * 130) as i32;
        value -= ((black_king & CB_SAFE_KING).count_ones() * 130) as i32;
    }

    let defending_king: u64;
    if b.side_to_move() == Color::White {
        defending_king = black_king;
        value *= -1;
    } else {
        defending_king = white_king;
    }
    if pieces_count < 8 {
        if value < 0 {
            // TODO value += distance(b.White.Kings, b.Black.Kings) * 20
            // TODO value += countFiguresMoves(b, bbDefendingKing) * 10
            value += ((defending_king & CB_CENTER_0).count_ones() * 80) as i32;
            value += ((defending_king & CB_CENTER_1).count_ones() * 40) as i32;
            value += ((defending_king & CB_BOARD_1).count_ones() * 10) as i32;
            value -= ((defending_king & CB_BOARD_0).count_ones() * 50) as i32;
        } else {
            // TODO value -= distance(b.White.Kings, b.Black.Kings) * 20
            // TODO value -= countFiguresMoves(b, bbDefendingKing) * 10
            value -= ((defending_king & CB_CENTER_0).count_ones() * 80) as i32;
            value -= ((defending_king & CB_CENTER_1).count_ones() * 40) as i32;
            value -= ((defending_king & CB_BOARD_1).count_ones() * 10) as i32;
            value += ((defending_king & CB_BOARD_0).count_ones() * 50) as i32;
        }
    }
    return value;
}
