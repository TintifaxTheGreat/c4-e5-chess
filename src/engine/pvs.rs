use super::{
    constants::{MATE, MIN_INT},
    evaluate,
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
    pub history: History,
    pub node_count: u64,
    pub store: Store,
}

impl Pvs {
    pub fn new() -> Self {
        Self {
            history: History::new(),
            node_count: 0,
            store: Store::new(),
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
        let mut best_value: MoveScore = MIN_INT;
        let mut value: MoveScore;

        if !playing.load(Ordering::Relaxed) {
            return 0;
        }

        if board.status() != BoardStatus::Ongoing {
            if board.status() == BoardStatus::Checkmate {
                return -MATE - i32::from(depth);
            }
            return 0;
        }

        if self.history.get(&board) > 2 {
            return 0;
        }

        if depth < 1 {
            self.node_count += 1;
            return evaluate::evaluate(&board);
        }

        let children: Vec<AnnotatedMove> = match self.store.get(depth, &board) {
            Some((_, v, true)) => return v,
            Some((mv, _, false)) => MoveGen::get_legal_sorted(&board, Some(mv)),
            None => MoveGen::get_legal_sorted(&board, None),
        };

        let moves = children.iter();
        let mut bresult = mem::MaybeUninit::<Board>::uninit();

        for (i, child) in &mut moves.enumerate() {
            let _ = &board.make_move(child.mv, unsafe { &mut *bresult.as_mut_ptr() });
            self.history.inc(unsafe { &*bresult.as_ptr() });
            if i == 0 {
                best_value = -self.execute(
                    unsafe { *bresult.as_ptr() },
                    depth - 1,
                    -beta,
                    -alpha,
                    playing,
                )
            } else {
                value = -self.execute(
                    unsafe { *bresult.as_ptr() },
                    depth - 1,
                    -alpha - 1,
                    -alpha,
                    playing,
                );
                if value > best_value {
                    if alpha < value && value < beta {
                        best_value = -self.execute(
                            unsafe { *bresult.as_ptr() },
                            depth - 1,
                            -beta,
                            -value,
                            playing,
                        )
                    } else if value > best_value {
                        best_value = value;
                    }
                }
            }
            self.history.dec(unsafe { &*bresult.as_ptr() });

            if best_value >= beta {
                best_move = Some(child.mv);
                break;
            }
            if best_value > alpha {
                alpha = best_value;
                best_move = Some(child.mv);
            }
        }

        if let Some(bm) = best_move {
            self.store.put(depth - 1, best_value, &board, &bm);
        }
        best_value
    }
}

impl Default for Pvs {
    fn default() -> Self {
        Self::new()
    }
}
