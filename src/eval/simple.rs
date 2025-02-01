use super::{constants::*, evaluation::Evaluation, helpers::*};
use crate::misc::types::*;
use cozy_chess::{Board, Color, Piece};

#[derive(Default)]
pub struct Simple {}

impl Evaluation for Simple {
    /// A simple static evaluation function for the given board position.
    /// It's purpose is, to serve as a baseline for more sophisticated evaluation functions.
    fn evaluate(b: &Board) -> MoveScore {
        let mut value: MoveScore = 0;
        let pieces_count = b.occupied().len();

        let b_open_files = open_files(b);
        let b_half_open_files = half_open_files(b);

        let (color_attack, color_defend, rank1, rank2, rank3, rank6, rank7, rank8) =
            if b.side_to_move() == Color::White {
                (
                    Color::White,
                    Color::Black,
                    CB_RANK_1,
                    CB_RANK_2,
                    CB_RANK_3,
                    CB_RANK_6,
                    CB_RANK_7,
                    CB_RANK_8,
                )
            } else {
                (
                    Color::Black,
                    Color::White,
                    CB_RANK_8,
                    CB_RANK_7,
                    CB_RANK_6,
                    CB_RANK_3,
                    CB_RANK_2,
                    CB_RANK_1,
                )
            };

        // Rules concerning pawns
        value += (b.colored_pieces(color_attack, Piece::Pawn).0.count_ones() * 200) as MoveScore;
        value -= (b.colored_pieces(color_defend, Piece::Pawn).0.count_ones() * 200) as MoveScore;

        value += ((b.colored_pieces(color_attack, Piece::Pawn).0 & CB_CENTER_0_GOOD).count_ones()
            * 15) as MoveScore;
        value -= ((b.colored_pieces(color_defend, Piece::Pawn).0 & CB_CENTER_0_GOOD).count_ones()
            * 15) as MoveScore;

        value += ((b.colored_pieces(color_attack, Piece::Pawn).0 & CB_CENTER_1).count_ones() * 30)
            as MoveScore;
        value -= ((b.colored_pieces(color_defend, Piece::Pawn).0 & CB_CENTER_1).count_ones() * 30)
            as MoveScore;

        value += ((b.colored_pieces(color_attack, Piece::Pawn).0 & rank6).count_ones() * 50)
            as MoveScore;
        value -= ((b.colored_pieces(color_defend, Piece::Pawn).0 & rank3).count_ones() * 50)
            as MoveScore;

        value += ((b.colored_pieces(color_attack, Piece::Pawn).0 & rank7).count_ones() * 650)
            as MoveScore;
        value -= ((b.colored_pieces(color_defend, Piece::Pawn).0 & rank2).count_ones() * 650)
            as MoveScore;

        value -=
            (multiple_on_file(b.colored_pieces(color_attack, Piece::Pawn).0) * 30) as MoveScore;
        value +=
            (multiple_on_file(b.colored_pieces(color_defend, Piece::Pawn).0) * 30) as MoveScore;

        // Rules concerning knights
        value += (b.colored_pieces(color_attack, Piece::Knight).0.count_ones() * 600) as MoveScore;
        value -= (b.colored_pieces(color_defend, Piece::Knight).0.count_ones() * 600) as MoveScore;

        value -= ((b.colored_pieces(color_attack, Piece::Knight).0 & CB_BOARD_0).count_ones() * 29)
            as MoveScore;
        value += ((b.colored_pieces(color_defend, Piece::Knight).0 & CB_BOARD_0).count_ones() * 29)
            as MoveScore;

        // Rules concerning bishops
        value += (b.colored_pieces(color_attack, Piece::Bishop).0.count_ones() * 620) as MoveScore;
        value -= (b.colored_pieces(color_defend, Piece::Bishop).0.count_ones() * 620) as MoveScore;

        // Rules concerning rooks
        value += (b.colored_pieces(color_attack, Piece::Rook).0.count_ones() * 950) as MoveScore;
        value -= (b.colored_pieces(color_defend, Piece::Rook).0.count_ones() * 950) as MoveScore;

        value += ((b.colored_pieces(color_attack, Piece::Rook).0 & b_open_files).count_ones() * 40)
            as MoveScore;
        value -= ((b.colored_pieces(color_defend, Piece::Rook).0 & b_open_files).count_ones() * 40)
            as MoveScore;

        value += ((b.colored_pieces(color_attack, Piece::Rook).0 & b_half_open_files).count_ones()
            * 10) as MoveScore;
        value -= ((b.colored_pieces(color_defend, Piece::Rook).0 & b_half_open_files).count_ones()
            * 10) as MoveScore;

        value += ((b.colored_pieces(color_attack, Piece::Rook).0 & rank7).count_ones() * 80)
            as MoveScore;
        value -= ((b.colored_pieces(color_defend, Piece::Rook).0 & rank2).count_ones() * 80)
            as MoveScore;

        // Rules concerning queens
        value += (b.colored_pieces(color_attack, Piece::Queen).0.count_ones() * 1800) as MoveScore;
        value -= (b.colored_pieces(color_defend, Piece::Queen).0.count_ones() * 1800) as MoveScore;

        value -= ((b.colored_pieces(color_attack, Piece::Queen).0 & CB_CENTER).count_ones() * 30)
            as MoveScore;
        value += ((b.colored_pieces(color_defend, Piece::Queen).0 & CB_CENTER).count_ones() * 30)
            as MoveScore;

        if pieces_count > 20 {
            value -= ((b.colored_pieces(color_attack, Piece::Knight).0 & rank1).count_ones() * 51)
                as MoveScore;
            value += ((b.colored_pieces(color_defend, Piece::Knight).0 & rank8).count_ones() * 51)
                as MoveScore;

            value -= ((b.colored_pieces(color_attack, Piece::Bishop).0 & rank1).count_ones() * 100)
                as MoveScore;
            value += ((b.colored_pieces(color_defend, Piece::Bishop).0 & rank8).count_ones() * 100)
                as MoveScore;

            value += ((b.colored_pieces(color_attack, Piece::Bishop).0 & CB_GOOD_BISHOP)
                .count_ones()
                * 20) as MoveScore;
            value -= ((b.colored_pieces(color_defend, Piece::Bishop).0 & CB_GOOD_BISHOP)
                .count_ones()
                * 20) as MoveScore;

            value += ((b.colored_pieces(color_attack, Piece::Queen).0 & CB_GOOD_QUEEN).count_ones()
                * 30) as MoveScore;
            value -= ((b.colored_pieces(color_defend, Piece::Queen).0 & CB_GOOD_QUEEN).count_ones()
                * 30) as MoveScore;

            value += ((b.colored_pieces(color_attack, Piece::King).0 & CB_SAFE_KING).count_ones()
                * 150) as MoveScore;
            value -= ((b.colored_pieces(color_defend, Piece::King).0 & CB_SAFE_KING).count_ones()
                * 150) as MoveScore;
        }

        if pieces_count < 8 {
            let mut kings_value: MoveScore = kings_distance(b) * -10;
            kings_value -= defending_kings_moves_count(b) as MoveScore * 10;
            kings_value -= ((b.colored_pieces(color_defend, Piece::King).0 & CB_CENTER_0)
                .count_ones()
                * 80) as MoveScore;
            kings_value -= ((b.colored_pieces(color_defend, Piece::King).0 & CB_CENTER_1)
                .count_ones()
                * 40) as MoveScore;
            kings_value -= ((b.colored_pieces(color_defend, Piece::King).0 & CB_BOARD_1)
                .count_ones()
                * 10) as MoveScore;
            kings_value += ((b.colored_pieces(color_defend, Piece::King).0 & CB_BOARD_0)
                .count_ones()
                * 50) as MoveScore;
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
    fn test_evaluate() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_str(fen).unwrap();
        let score = Simple::evaluate(&board);
        assert_eq!(score, 0); // Initial position should be balanced

        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
        let board = Board::from_str(fen).unwrap();
        let score = Simple::evaluate(&board);
        assert_eq!(score, -30); // White has an advantage with an extra pawn in the center, but it's black's turn

        let fen = "rnbqkbnr/ppp1pppp/8/8/3Pp3/8/PPP2PPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_str(fen).unwrap();
        let score = Simple::evaluate(&board);
        assert!(score < 0); // Black has an advantage with an extra pawn in the center, but it's white's turn

        let fen = "rn1qk1nr/pppbbppp/8/3pp3/3PP3/1P4P1/PBP2PBP/RN1QK1NR w KQkq - 0 1";
        let board = Board::from_str(fen).unwrap();
        let score = Simple::evaluate(&board);
        assert!(score > 0); // White has positional advantage and it's white's turn

        let fen = "rn1qk1nr/pppbbppp/8/3pp3/3PP3/1P4P1/PBP2PBP/RN1QK1NR b KQkq - 0 1";
        let board = Board::from_str(fen).unwrap();
        let score = Simple::evaluate(&board);
        assert!(score < 0); // White has positional advantage but it's black's turn

        let fen = "rn1qk1nr/pbp2pbp/1p4p1/3pp3/3PP3/8/PPPBBPPP/RN1QK1NR w KQkq - 0 1";
        let board = Board::from_str(fen).unwrap();
        let score = Simple::evaluate(&board);
        assert!(score < 0); // Black has positional advantage but it's white's turn

        let fen = "rn1qk1nr/pbp2pbp/1p4p1/3pp3/3PP3/8/PPPBBPPP/RN1QK1NR b KQkq - 0 1";
        let board = Board::from_str(fen).unwrap();
        let score = Simple::evaluate(&board);
        assert!(score > 0); // Black has positional advantage and it's black's turn

        let fen_n = "8/8/4k3/8/3R4/4K3/8/8 w - - 0 1";
        let fen_f = "8/8/4k3/8/3R4/8/8/4K3 w - - 0 1";
        let board_n = Board::from_str(fen_n).unwrap();
        let board_f = Board::from_str(fen_f).unwrap();
        let score_n = Simple::evaluate(&board_n);
        let score_f = Simple::evaluate(&board_f);
        assert!(score_n > score_f); // Nearer king is better for stronger party

        let fen_n = "8/8/4k3/8/3R4/4K3/8/8 b - - 0 1";
        let fen_f = "8/8/4k3/8/3R4/8/8/4K3 b - - 0 1";
        let board_n = Board::from_str(fen_n).unwrap();
        let board_f = Board::from_str(fen_f).unwrap();
        let score_n = Simple::evaluate(&board_n);
        let score_f = Simple::evaluate(&board_f);
        assert!(score_n < score_f); // Nearer king is worse for weaker party

        let fen_n = "8/8/5b2/5b2/5k2/8/8/5K2 b - - 0 1";
        let fen_f = "8/5k2/5b2/5b2/8/8/8/5K2 b - - 0 1";
        let board_n = Board::from_str(fen_n).unwrap();
        let board_f = Board::from_str(fen_f).unwrap();
        let score_n = Simple::evaluate(&board_n);
        let score_f = Simple::evaluate(&board_f);
        assert!(score_n > score_f); // Nearer king is better for stronger party

        let fen_n = "8/8/5b2/5b2/5k2/8/8/5K2 w - - 0 1";
        let fen_f = "8/5k2/5b2/5b2/8/8/8/5K2 w - - 0 1";
        let board_n = Board::from_str(fen_n).unwrap();
        let board_f = Board::from_str(fen_f).unwrap();
        let score_n = Simple::evaluate(&board_n);
        let score_f = Simple::evaluate(&board_f);
        assert!(score_n < score_f); // Nearer king is worse for weaker party
    }
}
