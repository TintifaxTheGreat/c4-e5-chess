use super::{
    constants::*, history::History, move_gen::MoveGenPrime, pvs::Pvs, store::Store, types::*,
};
use chess::{Board, ChessMove, MoveGen};
use core::time::Duration;
use log::info;
use rayon::prelude::*;
use std::{
    cmp::{max, min},
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
    pub node_count: u64,
    pub game_store: Store,
    pub game_history: History,
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
                node_count: 0,
                game_store: Store::new(),
                game_history: History::new(),
            },
            Err(_) => panic!("FEN not valid"),
        }
    }
    pub fn set_timer(&mut self) -> JoinHandle<()> {
        self.playing.store(true, Ordering::Relaxed);
        let playing_clone = self.playing.clone();
        let move_time = self.move_time;
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(move_time));
            playing_clone.store(false, Ordering::Relaxed);
        })
    }

    pub fn find_move(&mut self) -> Option<ChessMove> {
        fn stabilise_search_results(
            old: &[AnnotatedMove],
            new: &[AnnotatedMove],
        ) -> Vec<AnnotatedMove> {
            let mut new_stabilised: Vec<AnnotatedMove> = new.to_owned();

            let diff_mean = new_stabilised
                .iter()
                .enumerate()
                .fold(0, |acc, (i, v)| acc + (old[i].sc - v.sc))
                / new_stabilised.len() as i32;

            new_stabilised.iter_mut().enumerate().for_each(|(i, v)| {
                v.sc = min(v.sc + diff_mean, old[i].sc);
            });
            new_stabilised
        }

        let alpha = MIN_INT;
        let beta = MAX_INT;
        let mut current_depth: Depth = 0;
        let mut best_move: Option<ChessMove> = None;
        let mut best_value: MoveScore = MIN_INT;
        let mut worst_value: MoveScore;
        let mut prior_values = MoveGen::get_legal_sorted(&self.board, None);
        let mut prior_values_old: Vec<AnnotatedMove> = vec![];

        self.set_timer();

        if prior_values.len() == 1 {
            return Some(prior_values[0].mv);
        }

        while current_depth <= self.max_depth {
            prior_values.par_iter_mut().for_each(
                |AnnotatedMove {
                     mv, sc, node_count, ..
                 }| {
                    let mut bresult = mem::MaybeUninit::<Board>::uninit();
                    let mut pvs = Pvs::new();
                    pvs.store.h.clone_from(&self.game_store.h);
                    pvs.history.h.clone_from(&self.game_history.h);

                    self.board
                        .make_move(*mv, unsafe { &mut *bresult.as_mut_ptr() });
                    pvs.history.inc(unsafe { &*bresult.as_ptr() });
                    *sc = -pvs.execute(
                        unsafe { *bresult.as_ptr() },
                        current_depth,
                        -beta,
                        -alpha,
                        &self.playing,
                    );
                    pvs.history.dec(unsafe { &*bresult.as_ptr() });
                    *node_count = pvs.node_count;
                },
            );

            if !self.playing.load(Ordering::Relaxed) {
                info!("Time has expired");
                break;
            }

            if current_depth % 2 == 1 {
                prior_values = stabilise_search_results(&prior_values_old, &prior_values);
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
            // Add to node count
            self.node_count += prior_values.iter().fold(
                0,
                |acc,
                 AnnotatedMove {
                     mv: _,
                     sc: _,
                     node_count: nc,
                     ..
                 }| acc + nc,
            );

            // Forward pruning
            if current_depth >= FORWARD_PRUNING_DEPTH_START {
                let moves_count = prior_values.len();

                worst_value = prior_values[moves_count - 1].sc;
                if worst_value < best_value {
                    let cut_index =
                        max(FORWARD_PRUNING_MINIMUM, moves_count / FORWARD_PRUNING_RATIO);
                    info!("cut at {}", cut_index);
                    prior_values.truncate(cut_index);
                }
            } //TODO remove debugging code
            prior_values
                .iter()
                .for_each(|AnnotatedMove { mv, sc, .. }| {
                    info!("....{0} {1}", mv.to_string(), sc,);
                });
            info!(
                "Current Depth: {0}, Node Count: {1}",
                current_depth, self.node_count
            );

            current_depth += 1;
            prior_values_old = prior_values.clone();
        }
        self.game_store.put(
            current_depth - 1,
            best_value,
            &self.board,
            &best_move.unwrap(),
        );
        best_move
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new(String::from(""), 0, 0)
    }
}
