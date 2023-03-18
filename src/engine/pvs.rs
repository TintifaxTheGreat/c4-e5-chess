use super::{constants::MATE, evaluate::evaluate, move_gen::MoveGenPrime, store::Store, types::*};
use chess::{Board, BoardStatus, ChessMove, MoveGen};
use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};

pub fn pvs(
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
    let children: Vec<AnnotatedMove>;
    let mut value;
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
        *node_count += 1;
        return evaluate(&board);
        //return quiesce::quiesce(board, alpha, beta, playing, stop_time, node_count);
    }

    let mut moves = children.iter();
    let child_1 = moves.next().unwrap();
    let mut bresult = mem::MaybeUninit::<Board>::uninit();
    unsafe {
        let _ = &board.make_move(child_1.mv, &mut *bresult.as_mut_ptr());
    }
    unsafe {
        value = -pvs(
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

    for child in &mut moves {
        if value >= beta {
            alpha = value;
            best_move = Some(child.mv);
            break;
        }
        if value > alpha {
            alpha = value;
            best_move = Some(child.mv);
        }

        bresult = mem::MaybeUninit::<Board>::uninit();
        unsafe {
            let _ = &board.make_move(child.mv, &mut *bresult.as_mut_ptr());
        }
        unsafe {
            temp = -pvs(
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
                bresult = mem::MaybeUninit::<Board>::uninit(); // TODO this can be removed
                unsafe {
                    let _ = &board.make_move(child.mv, &mut *bresult.as_mut_ptr());
                }
                unsafe {
                    value = -pvs(
                        *bresult.as_ptr(),
                        store,
                        depth - 1,
                        -beta,
                        -temp,
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

    if best_move.is_some() {
        store.put(depth - 1, alpha, &board, &best_move.unwrap());
    }

    return value;
}
