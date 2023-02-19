use std::time::Duration;

use chess::Color;

use crate::engine::game::Game;

use super::constants::*;

pub struct TimeManagement {
    pub white_time: u16,
    pub black_time: u16,
    pub white_inc: u16,
    pub black_inc: u16,
    pub moves_to_go: u16,
}

impl TimeManagement {
    fn set_game_time(&mut self, g: &mut Game) {
        let time_for_all_moves: u16;
        let mut time_usage_percent = TIME_USAGE_PERCENT_DEFAULT;

        if self.white_time == 0 {
            self.moves_to_go = ESTIMATED_REST_MOVES;
        }

        if g.board.side_to_move() == Color::White {
            time_for_all_moves = self.white_time + (self.moves_to_go - 1) * self.white_inc
        } else {
            time_for_all_moves = self.black_time + (self.moves_to_go - 1) * self.black_inc
        }

        // TODO check if this is correct
        if g.move_number < EARLY_GAME_MAX_MOVES && self.moves_to_go > EARLY_GAME_MIN_REST_MOVES {
            time_usage_percent = TIME_USAGE_PERCENT_EARLY_GAME;
        }

        let new_time_ms = (time_for_all_moves * time_usage_percent) / (self.moves_to_go * 100);
        g.move_time = Duration::from_millis(u64::from(new_time_ms));
    }
}

impl Default for TimeManagement {
    fn default() -> TimeManagement {
        TimeManagement {
            white_time: 0,
            black_time: 0,
            white_inc: 0,
            black_inc: 0,
            moves_to_go: 0,
        }
    }
}
