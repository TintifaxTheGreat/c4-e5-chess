use super::{constants::*, pvs::Pvs, store::Store, types::*};
use chess::{Board, ChessMove, MoveGen};
use core::time::Duration;
use log::info;
use rayon::prelude::*;
use std::{
    mem,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

pub struct Game {
    pub max_depth: Depth,
    pub board: Board,
    pub move_time: MoveTime, // in Milliseconds
    pub move_number: MoveNumber,
    pub playing: Arc<AtomicBool>,
    pub nodes_count: u64,
    pub game_store: Store,
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
                nodes_count: 0,
                game_store: Store::new(),
            },
            Err(_) => panic!("FEN not valid"),
        }
    }
    pub fn set_timer(&mut self) -> JoinHandle<()> {
        self.playing.store(true, Ordering::Relaxed);
        let playing_clone = self.playing.clone();
        let move_time = self.move_time;
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(move_time));
            playing_clone.store(false, Ordering::Relaxed);
        });

        self.nodes_count = 0; // TODO this belongs elsewhere

        handle
    }

    pub fn find_move(&mut self) -> Option<ChessMove> {
        let alpha = MIN_INT;
        let beta = MAX_INT;
        let mut current_depth: Depth = 0;
        let mut best_move: Option<ChessMove> = None;
        let mut best_value: MoveScore;
        let mut worst_value: MoveScore;
        let mut moves = MoveGen::new_legal(&self.board);

        self.set_timer();

        if moves.len() == 1 {
            return Some(moves.next().unwrap());
        }

        let mut prior_values: Vec<ScoredMove> = moves.map(|mv| ScoredMove { mv, sc: 0 }).collect();
        while current_depth <= self.max_depth {
            if !self.playing.load(Ordering::Relaxed) {
                info!("Time has expired (1)");
                break;
            }

            prior_values
                .par_iter_mut()
                .for_each(|ScoredMove { mv, sc }| {
                    let mut bresult = mem::MaybeUninit::<Board>::uninit();
                    let mut pvs = Pvs::new();
                    pvs.store.h.clone_from(&self.game_store.h);

                    pvs.history.inc(&self.board);

                    self.board
                        .make_move(*mv, unsafe { &mut *bresult.as_mut_ptr() });
                    *sc = -pvs.execute(
                        unsafe { *bresult.as_ptr() },
                        current_depth,
                        -beta,
                        -alpha,
                        &self.playing,
                    );

                    pvs.history.dec(&self.board);
                });

            if !self.playing.load(Ordering::Relaxed) {
                info!("Time has expired (2)");
                break;
            }

            prior_values.sort_by(|a, b| b.sc.cmp(&a.sc));

            best_move = Some(prior_values[0].mv);
            best_value = prior_values[0].sc;
            if best_value > MATE_LEVEL {
                info!(
                    "Mate level was reached. Best move was {}",
                    best_move.unwrap().to_string()
                );
                break;
            }

            // Forward pruning
            if current_depth >= LATE_PRUNING_DEPTH_START {
                let moves_count = prior_values.len();
                let mut cut_index = moves_count;
                worst_value = prior_values[moves_count - 1].sc;
                if worst_value < best_value {
                    for (i, pv) in prior_values.iter().enumerate().skip(3) {
                        if (100 * (pv.sc - worst_value) / (best_value - worst_value))
                            < LATE_PRUNING_PERCENT
                        {
                            cut_index = i;
                            info!("cut at {}", i);
                            break;
                        }
                    }
                    prior_values.truncate(cut_index);
                }
            } //TODO remove debugging code
            prior_values.iter().for_each(|ScoredMove { mv, sc }| {
                info!("....{0} {1}", mv.to_string(), sc,);
            });
            info!(
                "Current Depth: {0}, Node Count: {1}",
                current_depth, self.nodes_count
            );

            current_depth += 1;
        }
        self.game_store
            .put(current_depth - 1, alpha, &self.board, &best_move.unwrap());
        best_move
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new(String::from(""), 0, 0)
    }
}
