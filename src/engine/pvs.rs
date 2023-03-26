use super::{
    constants::{MATE, MIN_INT},
    evaluate::evaluate,
    history::History,
    move_gen::MoveGenPrime,
    store::Store,
    types::*,
};
use chess::{Board, BoardStatus, ChessMove, MoveGen};
use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

pub struct Pvs {
    pub store: Store,
    pub history: History,
    pub node_count: u64,
}

impl Pvs {
    pub fn new() -> Self {
        Self {
            store: Store::new(),
            history: History::new(),
            node_count: 0,
        }
    }

    pub fn execute(
        &mut self,
        board: Board,
        depth: Depth,
        mut alpha: MoveScore,
        beta: MoveScore,
        playing: &Arc<AtomicBool>,
    ) -> MoveScore {
        let mut best_move: Option<ChessMove> = None;
        let mut score: MoveScore = MIN_INT;
        let mut value: MoveScore;

        if !playing.load(Ordering::Relaxed) {
            return 0;
        }

        if self.history.get(&board) > 2 {
            return 0;
        }

        let children: Vec<AnnotatedMove> = match self.store.get(depth, &board) {
            Some((_, v, true)) => return v,
            Some((mv, _, false)) => MoveGen::get_legal_sorted(&board, false, Some(mv)),
            None => MoveGen::get_legal_sorted(&board, false, None),
        };

        if children.is_empty() {
            if board.status() == BoardStatus::Checkmate {
                return -MATE - i32::from(depth);
            }
            return 0;
        }

        if depth < 1 {
            self.node_count += 1;
            return evaluate(&board);
            // TODO: Quiescence search causes issues
            // return quiesce::quiesce(board, alpha, beta, playing, stop_time, node_count);
        }

        let moves = children.iter();
        let mut bresult = mem::MaybeUninit::<Board>::uninit();

        for (i, child) in &mut moves.enumerate() {
            self.history.inc(&board);
            unsafe {
                let _ = &board.make_move(child.mv, &mut *bresult.as_mut_ptr());
            }
            if i == 0 {
                unsafe {
                    score = -self.execute(*bresult.as_ptr(), depth - 1, -beta, -alpha, playing)
                }
            } else {
                unsafe {
                    value = -self.execute(*bresult.as_ptr(), depth - 1, -alpha - 1, -alpha, playing)
                }

                if value > score {
                    if alpha < value && value < beta && depth > 2 {
                        unsafe {
                            score =
                                -self.execute(*bresult.as_ptr(), depth - 1, -beta, -value, playing)
                        }
                    } else {
                        score = value;
                    }
                }
            }
            self.history.dec(&board);

            if score >= beta {
                best_move = Some(child.mv);
                break;
            }
            if score > alpha {
                alpha = score;
                best_move = Some(child.mv);
            }
        }

        if let Some(bm) = best_move {
            self.store.put(depth - 1, score, &board, &bm);
        }
        score
    }
}
