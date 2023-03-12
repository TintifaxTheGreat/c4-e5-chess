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
        let mut iterable = MoveGen::new_legal(&board);
        let targets = board.color_combined(!board.side_to_move());

        iterable.set_iterator_mask(*targets);
        for mv in &mut iterable {
            result.push(AnnotatedMove { mv, capture: true });
        }

        if captures_only {
            return result;
        }

        iterable.set_iterator_mask(!EMPTY);
        for mv in &mut iterable {
            result.push(AnnotatedMove { mv, capture: false });
        }

        match old_move {
            Some(mv) => {
                let mut i = 0;
                for c in &mut result.iter() {
                    if c.mv == mv {
                        result.swap(0, i);
                        break;
                    }
                    i += 1;
                }
                return result;
            }
            None => return result,
        }
    }

    fn count_legal(board: &Board) -> usize {
        MoveGen::new_legal(&board).count()
    }
}
