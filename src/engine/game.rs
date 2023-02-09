use chess::Board;
use std::time::Duration;

use super::constants::{DEFAULT_TIME, INIT_MAX_DEPTH, INIT_QUIET_DEPTH, START_FEN};

struct Game {
    max_depth: u16,
    inc_quiet_depth: u16,
    board: Board,
    playing: bool,
    move_time: Duration,
    //TODO board_history:
}

impl Game {
    fn new(fen: String, max_depth: u16, inc_quiet_depth: u16, move_time: Duration) -> Self {
        let board = Board::from_fen(if fen.is_empty() {
            String::from(START_FEN)
        } else {
            fen
        });
        if board.is_some() {
            Self {
                max_depth: if max_depth == 0 {
                    INIT_MAX_DEPTH
                } else {
                    max_depth
                },
                inc_quiet_depth: if inc_quiet_depth == 0 {
                    INIT_QUIET_DEPTH
                } else {
                    inc_quiet_depth
                },
                board: board.unwrap(),
                playing: true,
                move_time: if move_time.is_zero() {
                    DEFAULT_TIME
                } else {
                    move_time
                },
            }
        } else {
            panic!("FEN not valid");
        }
    }
}
