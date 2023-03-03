use super::{
    constants::{INIT_QUIET_DEPTH, MATE, PVS_DEPTH},
    evaluate::evaluate,
    move_gen::MoveGenPrime,
    store::Store,
};
use chess::{Board, BoardStatus, ChessMove, MoveGen};
use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    }, time::SystemTime,
};

pub fn negamax(
    board: Board,
    store: &mut Store,
    depth: u16,
    mut alpha: i32,
    beta: i32,
    unsorted: bool,
    mut is_quiescence: bool,
    playing: &Arc<AtomicBool>,
    stop_time: SystemTime,
) -> (Option<ChessMove>, i32) {
    let mut best_move: Option<ChessMove> = None;
    let mut pvs = true;
    let mut children: Vec<(ChessMove, bool)> = Vec::new();

    match store.get(depth, &board) {
        Some((mm, v, fresh)) => {
            if fresh {
                return (Some(mm), v);
            } else {
                let mut i = 0;
                for c in &mut children.clone().into_iter() {
                    if c.0 == mm {
                        children.swap(0, i);
                        break;
                    }
                    i += 1;
                }
            }
        }
        None => children = MoveGen::get_legal_sorted(&board),
    }

    // TODO consider board history

    if children.len() == 0 {
        if board.status() == BoardStatus::Checkmate {
            // TODO store the mate value (?)
            return (None, -MATE - i32::from(depth));
        }
        return (best_move, 0);
    }

    if depth < 1 {
        return (best_move, evaluate(&board, 15)); //TODO change this
    }

    for c in &mut children.into_iter() {
        let mut value;
        let mut value_move: Option<ChessMove>;
        let mut bresult = mem::MaybeUninit::<Board>::uninit();

        unsafe {
            let _ = &board.make_move(c.0, &mut *bresult.as_mut_ptr());
        }

        // TODO add quiescence extension

        //let new_depth = depth - 1; // TODO change this
        let mut new_depth: u16 = 0;
        if !unsorted && pvs {
            new_depth += PVS_DEPTH;
        } else {
            if depth == 1 && c.1 && !is_quiescence {
                is_quiescence = true;
                new_depth = depth + INIT_QUIET_DEPTH - 1; // TODO change this
            } else {
                new_depth = depth - 1;
            }
        }

        if pvs {
            unsafe {
                (value_move, value) = negamax(
                    *bresult.as_ptr(),
                    store,
                    new_depth,
                    -beta,
                    -alpha,
                    true,
                    is_quiescence,
                    playing,
                    stop_time,
                );
                value *= -1;
            }
        } else {
            unsafe {
                (value_move, value) = negamax(
                    *bresult.as_ptr(),
                    store,
                    new_depth,
                    -alpha - 1,
                    -alpha,
                    true,
                    is_quiescence,
                    playing,
                    stop_time,
                );
                value *= -1;
            }
            if value > alpha {
                unsafe {
                    (value_move, value) = negamax(
                        *bresult.as_ptr(),
                        store,
                        new_depth,
                        -beta,
                        -alpha,
                        true,
                        is_quiescence,
                        playing,
                        stop_time,
                    );
                    value *= -1;
                }
            }
        }

        if value >= beta {
            return (best_move, beta);
        }

        if value > alpha {
            alpha = value;
            best_move = value_move;
            pvs = false;
        }
    }

    if (!playing.load(Ordering::Relaxed)) || ((SystemTime::now() >= stop_time)) {
        return (None, 0);
    }

    if best_move.is_some() {
        store.put(depth - 1, alpha, &board, &best_move.unwrap());
    }

    return (best_move, alpha);
}
