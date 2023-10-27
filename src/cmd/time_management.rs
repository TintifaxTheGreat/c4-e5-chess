use super::constants::*;
use crate::engine::game::Game;
use crate::misc::types::*;
use chess::Color;
use log::info;
use std::cmp::{max, min};

/// A structure holing available time and increment and the number of moves until next time control.
#[derive(Default)]
pub struct TimeManagement {
    pub white_time: MoveTime,
    pub black_time: MoveTime,
    pub white_inc: MoveTime,
    pub black_inc: MoveTime,
    pub moves_to_go: MoveNumber,
}

impl TimeManagement {
    /// Calculate time to be spent for the next move.
    pub fn set_game_time(&mut self, g: &mut Game) {
        fn move_time_fraction(move_number: MoveNumber) -> MoveTime {
            if move_number >= MOVE_LATE_GAME_START {
                MOVE_TIME_FRACTION_LATE_GAME
            } else {
                (MOVE_TIME_FRACTION_LATE_GAME - MOVE_TIME_FRACTION_EARLY_GAME) * move_number
                    / MOVE_LATE_GAME_START
                    + MOVE_TIME_FRACTION_EARLY_GAME
            }
        }

        let time_avail: MoveTime;
        let inc_avail: MoveTime;
        let mut move_time: MoveTime;

        if g.board.side_to_move() == Color::White {
            time_avail = self.white_time;
            inc_avail = self.white_inc;
        } else {
            time_avail = self.black_time;
            inc_avail = self.black_inc;
        }

        move_time = time_avail / move_time_fraction(g.move_number) + inc_avail / 2;
        move_time = min(move_time, time_avail - MIN_MOVE_TIME);
        move_time = max(move_time, MIN_MOVE_TIME);
        g.move_time = move_time;
        info!("Movetime was set to {}", move_time);
    }
}
