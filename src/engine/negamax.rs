use super::{
    constants::{MATE, MATE_LEVEL},
    evaluate::evaluate,
    move_gen::MoveGenPrime,
    quiesce,
    store::Store,
    types::*,
};
use chess::{Board, BoardStatus, ChessMove, MoveGen};
use log::info;
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
    playing: &Arc<AtomicBool>,
    stop_time: SystemTime,
    node_count: &mut u64,
) -> MoveScore {
    let mut best_move: Option<ChessMove> = None;
    let mut children: Vec<AnnotatedMove> = Vec::new();
    let mut value = 0;
    let mut temp: i32;

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
        //return evaluate(&board);
        return quiesce::quiesce(board, alpha, beta, playing, stop_time, node_count);
    }

    

    for (i, child) in &mut children.iter().enumerate() {
        let mut bresult = mem::MaybeUninit::<Board>::uninit();

        unsafe {
            let _ = &board.make_move(child.mv, &mut *bresult.as_mut_ptr());
        }

        if i == 0 {
            unsafe {
                value = -negamax(
                    *bresult.as_ptr(),
                    store,
                    depth - 1,
                    -beta,
                    -alpha,
                    playing,
                    stop_time,
                    node_count,
                )
            }
        } else {
            unsafe {
                temp = -negamax(
                    *bresult.as_ptr(),
                    store,
                    depth - 1,
                    -alpha - 1,
                    -alpha,
                    playing,
                    stop_time,
                    node_count,
                )
            }
            if temp > value {
                if alpha < temp && temp < beta && depth > 2 {
                    unsafe {
                        value = -negamax(
                            *bresult.as_ptr(),
                            store,
                            depth - 1,
                            -beta,
                            -alpha, //TODO was -temp
                            playing,
                            stop_time,
                            node_count,
                        )
                    }
                } else {
                    value = temp;
                }
            }
        }

        if value > MATE_LEVEL {
            return value;
        }

        if value >= beta {
            return value;
        }

        if value > alpha {
            alpha = value;
            best_move = Some(child.mv);
        }
    }

    if best_move.is_some() {
        store.put(depth - 1, alpha, &board, &best_move.unwrap());
    }

    return alpha;
}
