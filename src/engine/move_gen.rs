use super::types::*;
use chess::{Board, ChessMove, MoveGen, EMPTY};

pub trait MoveGenPrime {
    fn get_legal_sorted(
        board: &Board,
        captures_only: bool,
        old_move: Option<ChessMove>,
    ) -> Vec<AnnotatedMove>;
    fn count_legal(board: &Board) -> usize;
}

impl MoveGenPrime for MoveGen {
    fn get_legal_sorted(
        board: &Board,
        captures_only: bool,
        old_move: Option<ChessMove>,
    ) -> Vec<AnnotatedMove> {
        let mut result: Vec<AnnotatedMove> = Vec::new();
        let mut iterable = MoveGen::new_legal(board);
        let targets = board.color_combined(!board.side_to_move());

        iterable.set_iterator_mask(*targets);
        for mv in &mut iterable {
            result.push(AnnotatedMove {
                mv,
                sc: 0,
                capture: true,
                node_count: 0,
            });
        }

        if captures_only {
            return result;
        }

        iterable.set_iterator_mask(!EMPTY);
        for mv in &mut iterable {
            result.push(AnnotatedMove {
                mv,
                sc: 0,
                capture: false,
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

    fn count_legal(board: &Board) -> usize {
        MoveGen::new_legal(board).count()
    }
}
