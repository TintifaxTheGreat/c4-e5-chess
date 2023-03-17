use super::{
    evaluate::evaluate,
    types::*, move_gen::MoveGenPrime,
};
use chess::{Board,   MoveGen};
use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};

pub fn quiesce(
    board: Board,
    alpha: MoveScore,
    beta: MoveScore,
    playing: &Arc<AtomicBool>,
    stop_time: SystemTime,
    node_count: &mut u64,
) -> MoveScore {
    let children: Vec<AnnotatedMove>;
    let mut score: MoveScore;
    let mut alpha1 = alpha;

    if (!playing.load(Ordering::Relaxed)) || (SystemTime::now() >= stop_time) {
        return 0;
    }

    score = evaluate(&board);
    if score >= beta {
        return beta;
    }
    if score > alpha1 {
        alpha1 = score;
    }

    // TODO consider board history

    children = MoveGen::get_legal_sorted(&board, true, None);
    for child in &mut children.iter() {
        let mut bresult = mem::MaybeUninit::<Board>::uninit();
        unsafe {
            let _ = &board.make_move(child.mv, &mut *bresult.as_mut_ptr());
        }

        unsafe {
            score = -quiesce(
                *bresult.as_ptr(),
                -beta,
                -alpha1,
                playing,
                stop_time,
                node_count,
            );
        }

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha1 = score;
        }
    }
    return alpha1;
}