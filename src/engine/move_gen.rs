use crate::misc::types::*;
use chess::{Board, ChessMove, MoveGen, EMPTY};

/// A trait to extend the move generator of crate Chess.
pub trait MoveGenPrime {
    fn get_legal_sorted(board: &Board, old_move: Option<ChessMove>) -> Vec<AnnotatedMove>;
}

impl MoveGenPrime for MoveGen {
    /// Get all legal moves for given position, sort captures first.
    /// Also takes a proven good move ("old move") to be sorted first.
    fn get_legal_sorted(board: &Board, old_move: Option<ChessMove>) -> Vec<AnnotatedMove> {
        let mut result: Vec<AnnotatedMove> = Vec::new();
        let mut iterable = MoveGen::new_legal(board);
        let targets = board.color_combined(!board.side_to_move());

        iterable.set_iterator_mask(*targets);
        for mv in &mut iterable {
            result.push(AnnotatedMove {
                mv,
                sc: 0,
                node_count: 0,
            });
        }

        iterable.set_iterator_mask(!EMPTY);
        for mv in &mut iterable {
            result.push(AnnotatedMove {
                mv,
                sc: 0,
                node_count: 0,
            });
        }

        if let Some(mv) = old_move {
            for (i, c) in (&mut result.iter()).enumerate() {
                if c.mv == mv {
                    result.swap(0, i);
                    break;
                }
            }
        }
        result
    }
}
