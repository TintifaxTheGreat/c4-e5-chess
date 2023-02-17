use std::mem;

use chess::{Board, ChessMove, MoveGen};

use super::{evaluate::evaluate, store::Store};

pub fn negamax(
    board: Board,
    store: &mut Store,
    depth: u16,
    mut alpha: i32,
    beta: i32,
    unsorted: bool,
    is_quiescence: bool,
) -> (Option<ChessMove>, i32) {
    let mut best_move: Option<ChessMove> = None;
    match store.get(depth, &board) {
        Some((mm, v)) => return (Some(mm), v),
        None => {
            // TODO consider board history

            let mut children = MoveGen::new_legal(&board);
            if children.len() == 0 {
                // TODO consider king in check
                return (best_move, 0);
            }

            if depth < 1 {
                return (best_move, evaluate(&board, 15)); //TODO change this
            }

            // TODO add if unsorted stuff

            let mut best_move: Option<ChessMove> = None;
            let mut pvs = true;
            for c in &mut children {
                //println!("{}", c.to_string());
                let mut value = 0;
                let mut value_move: Option<ChessMove>;
                let mut bresult = mem::MaybeUninit::<Board>::uninit();

                unsafe {
                    &board.make_move(c, &mut *bresult.as_mut_ptr());
                }

                // TODO add quiescence extension

                let new_depth = depth - 1; // TODO change this
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
                            );
                            value *= -1;
                        }
                    }
                }

                if value >= beta {
                    return (best_move, beta); //TODO doesnt this look suspicious?
                }

                if value > alpha {
                    alpha = value;
                    best_move = value_move;
                    pvs = false;
                }
            }

            if best_move.is_some() {
                store.put(depth - 1, alpha, &board, &best_move.unwrap());
            }

            return (best_move, alpha);
        }
    }
}
