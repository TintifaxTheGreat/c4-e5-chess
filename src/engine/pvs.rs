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

pub fn pvs(
    board: Board,
    store: &mut Store,
    history: &mut History,
    depth: Depth,
    mut alpha: MoveScore,
    beta: MoveScore,
    playing: &Arc<AtomicBool>,
    node_count: &mut u64,
) -> MoveScore {
    let mut best_move: Option<ChessMove> = None;
    let children: Vec<AnnotatedMove>;
    let mut score: MoveScore = MIN_INT;
    let mut value: MoveScore;

    if !playing.load(Ordering::Relaxed) {
        return 0;
    }

    if history.get(&board) > 2 {
        return 0;
    }

    match store.get(depth, &board) {
        Some((_, v, true)) => return v,
        Some((mv, _, false)) => children = MoveGen::get_legal_sorted(&board, false, Some(mv)),
        None => children = MoveGen::get_legal_sorted(&board, false, None),
    }

    if children.len() == 0 {
        if board.status() == BoardStatus::Checkmate {
            return -MATE - i32::from(depth);
        }
        return 0;
    }

    if depth < 1 {
        *node_count += 1;
        return evaluate(&board);
        // TODO: Quiescence search causes issues
        // return quiesce::quiesce(board, alpha, beta, playing, stop_time, node_count);
    }

    let moves = children.iter();
    let mut bresult = mem::MaybeUninit::<Board>::uninit();

    for (i, child) in &mut moves.enumerate() {
        history.inc(&board);
        unsafe {
            let _ = &board.make_move(child.mv, &mut *bresult.as_mut_ptr());
        }
        if i == 0 {
            unsafe {
                score = -pvs(
                    *bresult.as_ptr(),
                    store,
                    history,
                    depth - 1,
                    -beta,
                    -alpha,
                    playing,
                    node_count,
                )
            }
        } else {
            unsafe {
                value = -pvs(
                    *bresult.as_ptr(),
                    store,
                    history,
                    depth - 1,
                    -alpha - 1,
                    -alpha,
                    playing,
                    node_count,
                )
            }

            if value > score {
                if alpha < value && value < beta && depth > 2 {
                    unsafe {
                        score = -pvs(
                            *bresult.as_ptr(),
                            store,
                            history,
                            depth - 1,
                            -beta,
                            -value,
                            playing,
                            node_count,
                        )
                    }
                } else {
                    score = value;
                }
            }
        }
        history.dec(&board);

        if score >= beta {
            best_move = Some(child.mv);
            break;
        }
        if score > alpha {
            alpha = score;
            best_move = Some(child.mv);
        }
    }
    // TODO check why storing takes that long (use hashing?)
    if best_move.is_some() {
        store.put(depth - 1, score, &board, &best_move.unwrap());
    }
    return score;
}
