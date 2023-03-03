use super::constants::*;
use crate::engine::game::Game;
use chess::Color;
use log::info;

pub struct TimeManagement {
    pub white_time: u64,
    pub black_time: u64,
    pub white_inc: u64,
    pub black_inc: u64,
    pub moves_to_go: u64,
}

impl TimeManagement {
    pub fn set_game_time(&mut self, g: &mut Game) {
        info!("im there at 1");

        let time_for_all_moves: u64;
        let mut time_usage_percent = TIME_USAGE_PERCENT_DEFAULT;
        info!("im there at 2");

        if self.moves_to_go == 0 {
            self.moves_to_go = ESTIMATED_REST_MOVES;
        }
        info!("im there at 3");

        if g.board.side_to_move() == Color::White {
            time_for_all_moves = self.white_time + (self.moves_to_go - 1) * self.white_inc;
            info!("im there at 4a");
        } else {
            time_for_all_moves = self.black_time + (self.moves_to_go - 1) * self.black_inc;
            info!("im there at 4b");
        }
        info!("moves to go: {}", self.moves_to_go);
        info!("time for all moves: {}", time_for_all_moves);
        info!("move number: {}", g.move_number);

        if (g.move_number < EARLY_GAME_MAX_MOVES) && (self.moves_to_go > EARLY_GAME_MIN_REST_MOVES) {
            time_usage_percent = TIME_USAGE_PERCENT_EARLY_GAME;
        }

        g.move_time = (time_for_all_moves * time_usage_percent) / (self.moves_to_go * 100);
        info!("Movetime was set to {}", g.move_time);
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
