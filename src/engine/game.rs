use crate::engine::negamax::negamax;

use super::{constants::*, store::Store};
use chess::{Board, ChessMove, MoveGen};
use std::{cell::Cell, mem, str::FromStr, time::Duration};

#[derive(Debug)]
pub enum ChessError {
    NoValidMoveFound,
    FenNotValid,
}

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
        match Board::from_str(if fen.is_empty() { START_FEN } else { &fen }) {
            Ok(board) => Self {
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
                board: board,
                playing: Cell::new(true),
                move_time: if move_time.is_zero() {
                    DEFAULT_TIME
                } else {
                    move_time
                },
            },
            Err(e) => panic!("FEN not valid"),
        }
    }

    pub fn find_move(&mut self) -> Option<ChessMove> {
        let mut store: Store = Store::new();
        let alpha = MIN_INT;
        let beta = MAX_INT;
        let mut current_depth: u16 = 0;
        let mut best_move: Option<ChessMove> = None;

        let mut moves = MoveGen::new_legal(&self.board);

        if moves.len() == 1 {
            return Some(moves.next().unwrap());
        }

        let mut prior_values: Vec<(ChessMove, i32)> = moves.map(|a| (a, 0)).collect();

        while current_depth <= self.max_depth {
            for i in 0..prior_values.len() {
                let mut bresult = mem::MaybeUninit::<Board>::uninit();
                unsafe {
                    let _ = &self
                        .board
                        .make_move(prior_values[i].0, &mut *bresult.as_mut_ptr());
                    match store.get(current_depth, &*bresult.as_ptr()) {
                        Some((_mm, vv, fresh)) => {
                            if fresh {
                                prior_values[i] = (prior_values[i].0, -vv);
                            } else {
                                // TODO make this more elegant
                                let (_mm, vv) = negamax(
                                    *bresult.as_ptr(),
                                    &mut store,
                                    current_depth,
                                    -beta,
                                    -alpha,
                                    false,
                                    false,
                                );
                                prior_values[i] = (prior_values[i].0, -vv);
                            }
                        }
                        None => {
                            let (_mm, vv) = negamax(
                                *bresult.as_ptr(),
                                &mut store,
                                current_depth,
                                -beta,
                                -alpha,
                                false,
                                false,
                            );
                            prior_values[i] = (prior_values[i].0, -vv);
                        }
                    }
                }
            }

            prior_values.sort_by(|a, b| b.1.cmp(&a.1));

            best_move = Some(prior_values[0].0.clone());
            println!("best was: {}", prior_values[0].0.to_string());
            let best_value = prior_values[0].1;
            if best_value > MATE_LEVEL {
                break;
            }

            // late pruning
            if current_depth > LATE_PRUNING_DEPTH {
                let mut cut_index = prior_values.len();
                for i in 0..prior_values.len() {
                    if prior_values[i].1 < best_value - LATE_PRUNING_THRESHOLD {
                        cut_index = i;
                        // TODO log.Print("cut at ", i)
                        break;
                    }
                }
                prior_values.truncate(cut_index);
            }

            current_depth += 1;
        }
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
        assert_eq!(count, 2);

        let board = Board::from_str("4kN2/4P3/7K/b5B1/2N2R2/6rn/2P5/8 b - - 0 1")?;
        let mut moves = MoveGen::new_legal(&board);
        let defending = board.color_combined(!board.side_to_move());
        moves.set_iterator_mask(*defending);
        let mut count = 0;
        for _ in &mut moves {
            count += 1;
        }
        assert_eq!(count, 3);

        Ok(())
    }

    #[test]
    fn test_play() {
        // Test 1
        let mut g = Game::new(
            "r1b2k1r/pppq3p/2np1p2/8/2B2B2/8/PPP3PP/4RR1K w - - 0 1".to_string(),
            4,
            0,
            Duration::new(5, 0),
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "f4h6"),
            None => panic!("No move found"),
        }

        // Test 2
        let mut g = Game::new(
            "1rb4r/pkPp3p/1b1P3n/1Q6/N3Pp2/8/P1P3PP/7K w - - 1 1".to_string(),
            4,
            0,
            Duration::new(5, 0),
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "b5d5"),
            None => panic!("No move found"),
        }

        // Test 3
        let mut g = Game::new(
            "8/2Q5/8/6q1/2K5/8/8/7k b - - 0 1".to_string(),
            4,
            0,
            Duration::new(5, 0),
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "g5c1"),
            None => panic!("No move found"),
        }

        // Test 4
        let mut g = Game::new(
            "2b3rk/1q3p1p/p1p1pPpQ/4N3/2pP4/2P1p1P1/1P4PK/5R2 w - - 1 1".to_string(),
            4,
            0,
            Duration::new(5, 0),
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "f1h1"),
            None => panic!("No move found"),
        }

        // Test 5
        let mut g = Game::new(
            "8/8/8/8/2R5/3k4/5K1n/8 w - - 0 1".to_string(),
            4,
            0,
            Duration::new(5, 0),
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "c4h4"),
            None => panic!("No move found"),
        }

        // Test 6
        
        let mut g = Game::new(
            "4r1k1/5bpp/2p5/3pr3/8/1B3pPq/PPR2P2/2R2QK1 b - - 0 1".to_string(),
            4, //TODO 4
            0,
            Duration::new(5, 0),
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "e5e1"),
            None => panic!("No move found"),
        }
        
    }
}
