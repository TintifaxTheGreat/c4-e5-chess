use crate::misc::types::*;
use cozy_chess::{Board, Move};
use std::ops::Not;

/// A trait to extend the move generator of crate Cozy Chess.
pub trait MoveGenPrime {
    fn get_legal_sorted(&self, old_move: Option<Move>) -> Vec<AnnotatedMove>;
}

impl MoveGenPrime for Board {
    /// Get all legal moves for given position, sort captures first.
    /// En passant captures are not considered.
    /// Also takes a proven good move ("old move") to be sorted first.
    fn get_legal_sorted(&self, old_move: Option<Move>) -> Vec<AnnotatedMove> {
        let mut result: Vec<AnnotatedMove> = Vec::new();
        let enemy_pieces = self.colors(!self.side_to_move());
        let other_squares = enemy_pieces.not();

        self.generate_moves(|moves| {
            let mut captures = moves;
            captures.to &= enemy_pieces;
            for mv in captures {
                result.push(AnnotatedMove {
                    mv,
                    sc: 0,
                    node_count: 0,
                });
            }

            let mut others = moves;
            others.to &= other_squares;
            for mv in others {
                result.push(AnnotatedMove {
                    mv,
                    sc: 0,
                    node_count: 0,
                });
            }

            false
        });

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
