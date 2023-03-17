use super::{
    constants::{MATE, MATE_LEVEL, MIN_INT},
    move_gen::MoveGenPrime,
    quiesce::quiesce,
    evaluate::evaluate,
    store::Store,
    types::*,
};
use chess::{Board, BoardStatus, MoveGen};
use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};

pub fn negascout(
    board: 
    Board,
    store: &mut Store,
    depth: Depth,
    alpha: MoveScore,
    beta: MoveScore,
    playing: &Arc<AtomicBool>,
    stop_time: SystemTime,
    node_count: &mut u64,
) -> MoveScore {
    let children: Vec<AnnotatedMove>;
    let (mut alpha_1, mut beta_1, mut score): (MoveScore, MoveScore, MoveScore);

    if (!playing.load(Ordering::Relaxed)) || (SystemTime::now() >= stop_time) {
        return 0;
    }

    match store.get(depth, &board) {
        Some((_, v, true)) => return v,
        Some((mv, _, false)) => children = MoveGen::get_legal_sorted(&board, false, Some(mv)),
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
        *node_count += 1;
        //return evaluate(&board);
        return quiesce(
            board,
            alpha,
            beta,
            playing,
            stop_time,
            node_count,
        );
    }

    let mut best_move = children[0].clone().mv;
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
                node_count,
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
                    node_count,
                );
            }
        }
        if score > alpha_1 {
            alpha_1 = score;
            best_move = child.mv;
        }

        if alpha_1 >= beta {
            store.put(depth - 1, alpha, &board, &best_move);
            return alpha_1;
        }

        beta_1 = alpha_1 + 1;
    }

    store.put(depth - 1, alpha, &board, &best_move);
    return alpha_1;
}
