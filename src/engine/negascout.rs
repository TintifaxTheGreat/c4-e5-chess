use super::{
    constants::{CAPTURE_DEPTH_INCREMENT, MATE, MATE_LEVEL, PVS_DEPTH, MIN_INT},
    evaluate::evaluate,
    move_gen::MoveGenPrime,
    store::Store,
    types::*,
};
use chess::{Board, BoardStatus, ChessMove, MoveGen};
use log::info;
use std::{
    cmp::max,
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};

pub fn negascout(
    board: Board,
    store: &mut Store,
    depth: Depth,
    alpha: MoveScore,
    beta: MoveScore,
    playing: &Arc<AtomicBool>,
    stop_time: SystemTime,
) -> MoveScore {
    let children: Vec<AnnotatedMove>;
    let (mut alpha_1, mut beta_1, mut score): (MoveScore, MoveScore, MoveScore);

    if (!playing.load(Ordering::Relaxed)) || (SystemTime::now() >= stop_time) {
        return 0;
    }

    match store.get(depth, &board) {
        Some((mv, v, fresh)) => {
            if fresh {
                return v;
            } else {
                children = MoveGen::get_legal_sorted(&board, false, Some(mv));
            }
        }
        None => children = MoveGen::get_legal_sorted(&board, false, None),
    }

    // TODO consider board history

    if children.len() == 0 {
        if board.status() == BoardStatus::Checkmate {
            return -MATE - i32::from(depth);
        }
        return 0;
    }

    if depth < 1 {
        return evaluate(&board);
    }

    (alpha_1, beta_1) = (MIN_INT, beta);
    for (i, child) in &mut children.iter().enumerate() {
        let mut bresult = mem::MaybeUninit::<Board>::uninit();
        unsafe {
            let _ = &board.make_move(child.mv, &mut *bresult.as_mut_ptr());
        }

        unsafe {
            score = -negascout(
                *bresult.as_ptr(),
                store,
                depth - 1,
                -beta_1,
                -alpha_1,
                playing,
                stop_time,
            );
        }

        if score > MATE_LEVEL {
            return score;
        }

        if i > 1 && depth > 1 && score > alpha_1 && score < beta {
            unsafe {
                alpha_1 = -negascout(
                    *bresult.as_ptr(),
                    store,
                    depth - 1,
                    -beta,
                    -score,
                    playing,
                    stop_time,
                );
            }
        }
        alpha_1 = max(alpha_1, score);

        if alpha_1 >= beta {
            return alpha_1;
            // TODO store this
        }

        beta_1 = alpha_1 + 1;
    }

    //store.put(depth - 1, alpha, &board, &alpha_1_move.unwrap());

    return alpha_1;
}
