use super::{constants::*, store::Item, store::Store};
use chess::{Board, ChessMove, MoveGen};
use std::{cell::Cell, mem, str::FromStr, time::Duration};

#[derive(Debug)]
pub enum ChessError {
    NoValidMoveFound,
    FenNotValid,
}

pub struct MoveValue {
    // TODO check if we really need this
    pub m: ChessMove,
    pub v: i32,
}

pub type MoveValueOption = Option<MoveValue>;
pub type MoveResult = Result<ChessMove, ChessError>;

pub struct Game {
    max_depth: u16,
    inc_quiet_depth: u16,
    pub board: Board,
    playing: Cell<bool>,
    move_time: Duration,
    //TODO board_history:
}

impl Game {
    pub fn new(fen: String, max_depth: u16, inc_quiet_depth: u16, move_time: Duration) -> Self {
        let board = Board::from_str(if fen.is_empty() { START_FEN } else { &fen });
        if !board.is_err() {
            Self {
                max_depth: if max_depth == 0 {
                    INIT_MAX_DEPTH
                } else {
                    max_depth
                },
                inc_quiet_depth: if inc_quiet_depth == 0 {
                    INIT_QUIET_DEPTH
                } else {
                    inc_quiet_depth
                },
                board: board.unwrap(),
                playing: Cell::new(true),
                move_time: if move_time.is_zero() {
                    DEFAULT_TIME
                } else {
                    move_time
                },
            }
        } else {
            panic!("FEN not valid");
        }
    }

    pub fn find_move(&mut self) -> MoveResult {
        let mut store: Store = Store::new();
        let mut result: Board = Board::default();
        let alpha = MIN_INT;
        let beta = MAX_INT;
        let current_depth: u16 = 0;
        let mut best_move: MoveResult = Err(ChessError::NoValidMoveFound);

        let mut moves = MoveGen::new_legal(&self.board);
        if moves.len() == 1 {
            return Ok(moves.next().unwrap());
        }

        while current_depth <= self.max_depth {
            let mut prior_values: Vec<(ChessMove, i32)> = Vec::new();
            for m in &mut moves {
                let mut bresult = mem::MaybeUninit::<Board>::uninit();
                unsafe {
                    &self.board.make_move(m, &mut *bresult.as_mut_ptr());
                    //let hop = store.get(current_depth, &*bresult.as_ptr());
                    match store.get(current_depth, &*bresult.as_ptr()) {
                        Some((mm, v)) => prior_values.push((mm, -v)),
                        None => {
                            let foo = &mut self.negamax(current_depth, -beta, -alpha, false, false);

                            //priorValues[move] *= -1
                        }
                    }

                    //result += MoveGen::movegen_perft_test(&*bresult.as_ptr(), depth - 1);
                }
                //&self.board.make_move(m, &mut result); //TODO
            }
        }

        return best_move;
    }

    fn negamax(
        &mut self,
        depth: u16,
        alpha: i32,
        beta: i32,
        unsorted: bool,
        is_quiescence: bool,
    ) -> MoveResult {
        let mut best_move: MoveResult = Err(ChessError::NoValidMoveFound);

        return best_move;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_move_gen() -> Result<(), chess::Error> {
        let board = Board::from_str("rnbqkbnr/p1ppp1p1/8/8/8/8/P1P1PPP1/RNBQKBNR w KQkq - 0 1")?;
        let mut moves = MoveGen::new_legal(&board);
        let defending = board.color_combined(!board.side_to_move());
        moves.set_iterator_mask(*defending);
        let mut count = 0;
        for _ in &mut moves {
            count += 1;
        }
        println!("{}", count);
        assert_eq!(count, 2);

        let board = Board::from_str("4kN2/4P3/7K/b5B1/2N2R2/6rn/2P5/8 b - - 0 1")?;
        let mut moves = MoveGen::new_legal(&board);
        let defending = board.color_combined(!board.side_to_move());
        moves.set_iterator_mask(*defending);
        let mut count = 0;
        for _ in &mut moves {
            count += 1;
        }
        println!("{}", count);
        assert_eq!(count, 3);

        Ok(())
    }
}
