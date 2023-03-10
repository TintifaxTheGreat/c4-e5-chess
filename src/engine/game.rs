use super::{constants::*, negamax::negamax, store::Store};

use chess::{Board, ChessMove, MoveGen};
use log::info;
use std::{
    cmp::max,
    mem,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, SystemTime},
};

pub type Depth = i16;
pub type MoveTime = u64;
pub type MoveNumber = u64;
pub type MoveScore = i32;

pub struct Game {
    pub max_depth: Depth,
    pub board: Board,
    pub move_time: MoveTime, // in Milliseconds
    pub move_number: MoveNumber,
    pub playing: Arc<AtomicBool>,
    //TODO board_history:
}

impl Game {
    pub fn new(fen: String, max_depth: Depth, move_time: MoveTime) -> Self {
        match Board::from_str(if fen.is_empty() { FEN_START } else { &fen }) {
            Ok(board) => Self {
                max_depth: if max_depth == 0 {
                    INIT_MAX_DEPTH
                } else {
                    max_depth
                },
                board,
                playing: Arc::new(AtomicBool::new(true)),
                move_time: if move_time == 0 {
                    DEFAULT_TIME
                } else {
                    move_time
                },
                move_number: 0,
            },
            Err(_) => panic!("FEN not valid"),
        }
    }

    pub fn find_move(&mut self) -> Option<ChessMove> {
        let mut store: Store = Store::new();
        let alpha = MIN_INT;
        let beta = MAX_INT;
        let mut current_depth: Depth = 0;
        let mut best_move: Option<ChessMove> = None;
        let mut best_value: MoveScore;
        let mut worst_value: MoveScore;
        let stop_time = SystemTime::now() + Duration::from_millis(self.move_time);

        let mut moves = MoveGen::new_legal(&self.board);

        if moves.len() == 1 {
            return Some(moves.next().unwrap());
        }

        let mut prior_values: Vec<(ChessMove, MoveScore, bool)> = moves.map(|a| (a, 0, true)).collect();

        'main_loop: while current_depth <= self.max_depth {
            let mut search: bool;
            let mut search_depth: Depth;
            for i in 0..prior_values.len() {
                let mut bresult = mem::MaybeUninit::<Board>::uninit();

                unsafe {
                    let _ = &self
                        .board
                        .make_move(prior_values[i].0, &mut *bresult.as_mut_ptr());
                    match store.get(current_depth, &*bresult.as_ptr()) {
                        Some((_mm, vv, fresh)) => {
                            if fresh {
                                prior_values[i] = (prior_values[i].0, -vv, true);
                                search = false;
                            } else {
                                search = true;
                            }
                        }
                        None => {
                            search = true;
                        }
                    }
                }

                if search {
                    search_depth = if prior_values[i].2 {
                        current_depth
                    } else {
                        max(0, current_depth - LATE_PRUNING_DEPTH_REDUCTION)
                    };
                    unsafe {
                        let (mut _mm, mut vv) = negamax(
                            *bresult.as_ptr(),
                            &mut store,
                            search_depth,
                            -beta,
                            -alpha,
                            false,
                            false,
                            &self.playing,
                            stop_time,
                        );
                        if !prior_values[i].2 && (-vv > alpha) {
                            (_mm, vv) = negamax(
                                *bresult.as_ptr(),
                                &mut store,
                                current_depth,
                                -beta,
                                -alpha,
                                false,
                                false,
                                &self.playing,
                                stop_time,
                            );
                        }
                        prior_values[i] = (prior_values[i].0, -vv, true);
                    }
                }

                if (!self.playing.load(Ordering::Relaxed)) || (SystemTime::now() >= stop_time) {
                    println!("time is not over");
                    break 'main_loop;
                }
            }

            prior_values.sort_by(|a, b| b.1.cmp(&a.1));

            best_move = Some(prior_values[0].0.clone());
            best_value = prior_values[0].1;
            if best_value > MATE_LEVEL {
                break;
            }

            // Late move pruning
            if current_depth >= LATE_PRUNING_DEPTH_START {
                let moves_count = prior_values.len();
                let mut cut_index = moves_count;
                worst_value = prior_values[moves_count - 1].1;
                if worst_value < best_value {
                    for i in 2..moves_count {
                        if (100 * (prior_values[i].1 - worst_value) / (best_value - worst_value))
                            < LATE_PRUNING_PERCENT
                        {
                            cut_index = i;
                            info!("cut at {}", i);
                            break;
                        }
                    }
                    prior_values.truncate(cut_index);
                }
            }

            // Late move reduction
            if current_depth >= LATE_MOVE_REDUCTION_DEPTH_START {
                for i in 0..prior_values.len() {
                    if i >= LATE_PRUNING_INDEX {
                        prior_values[i].2 = false;
                    }
                    info!(
                        "....{0} {1} {2}",
                        prior_values[i].0.to_string(),
                        prior_values[i].1,
                        prior_values[i].2,
                    );
                }
            }
            info!("Current Depth: {}", current_depth);
            current_depth += 1;
        }
        return best_move;
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new(String::from(""), 0, 0)
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
            5000,
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "f4h6"),
            None => panic!("No move found"),
        }

        // Test 2
        let mut g = Game::new(
            "1rb4r/pkPp3p/1b1P3n/1Q6/N3Pp2/8/P1P3PP/7K w - - 1 1".to_string(),
            4,
            5000,
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "b5d5"),
            None => panic!("No move found"),
        }

        // Test 3
        let mut g = Game::new("8/2Q5/8/6q1/2K5/8/8/7k b - - 0 1".to_string(), 4, 5000);
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "g5c1"),
            None => panic!("No move found"),
        }

        // Test 4
        let mut g = Game::new(
            "2b3rk/1q3p1p/p1p1pPpQ/4N3/2pP4/2P1p1P1/1P4PK/5R2 w - - 1 1".to_string(),
            4,
            5000,
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "f1h1"),
            None => panic!("No move found"),
        }

        // Test 5
        let mut g = Game::new("8/8/8/8/2R5/3k4/5K1n/8 w - - 0 1".to_string(), 4, 5000);
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "c4h4"),
            None => panic!("No move found"),
        }

        // Test 6
        let mut g = Game::new(
            "4r1k1/5bpp/2p5/3pr3/8/1B3pPq/PPR2P2/2R2QK1 b - - 0 1".to_string(),
            4,
            5000,
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "e5e1"),
            None => panic!("No move found"),
        }
        
        // Test 7
        let mut g = Game::new(
            "3q1rk1/4bp1p/1n2P2Q/1p1p1p2/6r1/Pp2R2N/1B1P2PP/7K w - - 1 0".to_string(),
            8,
            5000,
        );
        match g.find_move() {
            Some(m) => assert_eq!(m.to_string(), "h3g5"),
            None => panic!("No move found"),
        }
        
    }
}
