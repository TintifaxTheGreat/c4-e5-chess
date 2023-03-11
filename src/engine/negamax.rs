use super::{
    constants::{CAPTURE_DEPTH_INCREMENT, MATE, PVS_DEPTH},
    evaluate::evaluate,
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
    time::SystemTime,
};

pub fn negamax(
    board: Board,
    store: &mut Store,
    depth: Depth,
    mut alpha: MoveScore,
    beta: MoveScore,
    unsorted: bool,
    playing: &Arc<AtomicBool>,
    stop_time: SystemTime,
) -> (Option<ChessMove>, MoveScore) {
    let mut best_move: Option<ChessMove> = None;
    let mut children: Vec<AnnotatedMove> = Vec::new();
    let mut new_depth: Depth;

    if (!playing.load(Ordering::Relaxed)) || (SystemTime::now() >= stop_time) {
        return (None, 0);
    }

    match store.get(depth, &board) {
        Some((mv, v, fresh)) => {
            if fresh {
                return (Some(mv), v);
            } else {
                if unsorted {
                    let mut i = 0;
                    for c in &mut children.iter() {
                        if c.mv == mv {
                            children.swap(0, i);
                            break;
                        }
                        i += 1;
                    }
                }
            }
        }
        None => children = MoveGen::get_legal_sorted(&board, false),
    }

    // TODO consider board history

    if children.len() == 0 {
        if board.status() == BoardStatus::Checkmate {
            return (None, -MATE - i32::from(depth));
        }
        return (None, 0);
    }

    if depth < 1 {
        return (None, evaluate(&board));
    }

    for child in &mut children.into_iter() {
        let mut value;
        let value_move: Option<ChessMove>;
        let mut bresult = mem::MaybeUninit::<Board>::uninit();

        unsafe {
            let _ = &board.make_move(child.mv, &mut *bresult.as_mut_ptr());
        }

        new_depth = depth - 1;
        if new_depth == 0 && child.capture {
            new_depth = CAPTURE_DEPTH_INCREMENT;
        }

        unsafe {
            (value_move, value) = negamax(
                *bresult.as_ptr(),
                store,
                depth-1,
                -beta,
                -alpha,
                true,
                playing,
                stop_time,
            );
            value *= -1;
        }

        if value >= beta {
            return (best_move, beta);
        }

        if value > alpha {
            alpha = value;
            best_move = value_move;
        }
    }

    if best_move.is_some() {
        store.put(depth - 1, alpha, &board, &best_move.unwrap());
    }

    return (best_move, alpha);
}
