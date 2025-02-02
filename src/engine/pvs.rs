use super::{constants::*, history::History, move_gen::MoveGenPrime, store::Store};
use crate::eval::{evaluation::Evaluation, simple::Simple};
use crate::misc::types::*;
use cozy_chess::{Board, GameStatus, Move};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// A principal variant search.
pub struct Pvs {
    pub history: History,
    pub node_count: u64,
    pub store: Store,
}

impl Pvs {
    /// Constructor
    pub fn new() -> Self {
        Self {
            history: History::new(),
            node_count: 0,
            store: Store::new(),
        }
    }

    /// Execute the search given a board and parameters Alpha and Beta
    pub fn execute(
        &mut self,
        board: &Board,
        depth: Depth,
        mut alpha: MoveScore,
        beta: MoveScore,
        playing: &Arc<AtomicBool>,
        _capture: bool,
    ) -> MoveScore {
        let mut best_move: Option<Move> = None;

        if !playing.load(Ordering::Relaxed) {
            return 0;
        }

        if board.status() != GameStatus::Ongoing {
            if board.status() == GameStatus::Won {
                return -MATE - i32::from(depth);
            }
            return 0;
        }

        if self.history.get(board) > 2 {
            return 0;
        }

        if depth < 1 {
            self.node_count += 1;
            return Simple::evaluate(board);
        }

        let children: Vec<AnnotatedMove> = match self.store.get(depth, board) {
            Some((_, v, true)) => return v,
            Some((mv, _, false)) => board.get_legal_sorted(Some(mv)),
            None => board.get_legal_sorted(None),
        };

        for (i, child) in children.iter().enumerate() {
            let mut b1 = board.clone();
            b1.play_unchecked(child.mv);
            self.history.inc(&b1);

            let value = if i == 0 {
                -self.execute(&b1, depth - 1, -beta, -alpha, playing, child.cp)
            } else {
                let mut value =
                    -self.execute(&b1, depth - 1, -alpha - 1, -alpha, playing, child.cp);
                if value > alpha && value < beta {
                    value = -self.execute(&b1, depth - 1, -beta, -value, playing, child.cp);
                }
                value
            };

            self.history.dec(&b1);

            if value > alpha {
                alpha = value;
                best_move = Some(child.mv);
            }

            if alpha >= beta {
                break;
            }
        }

        if let Some(bm) = best_move {
            self.store.put(depth - 1, alpha, board, &bm);
        }

        alpha
    }
}

impl Default for Pvs {
    fn default() -> Self {
        Self::new()
    }
}
