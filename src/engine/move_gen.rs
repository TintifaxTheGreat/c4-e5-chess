use chess::{Board, ChessMove, MoveGen, EMPTY};

pub trait MoveGenPrime {
    fn get_legal_sorted(board: &Board) -> Vec<(ChessMove, bool)>;
    fn count_legal(board: &Board) -> usize;
}

impl MoveGenPrime for MoveGen {
    fn get_legal_sorted(board: &Board) -> Vec<(ChessMove, bool)> {
        let mut result: Vec<(ChessMove, bool)> = Vec::new();
        let mut iterable = MoveGen::new_legal(&board);
        let targets = board.color_combined(!board.side_to_move());

        iterable.set_iterator_mask(*targets);
        for m in &mut iterable {
            result.push((m, true));
        }

        iterable.set_iterator_mask(!EMPTY);
        for m in &mut iterable {
            result.push((m, false));
        }

        result
    }

    fn count_legal(board: &Board) -> usize {
        MoveGen::new_legal(&board).count()
    }
}
