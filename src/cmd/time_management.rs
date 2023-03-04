use std::cmp::min;

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
    // TODO fix me
    pub fn set_game_time(&mut self, g: &mut Game) {
        let time_for_all_moves: u64;
        let mut new_move_time: u64;
        let security_move_time: u64;

        let mut time_usage_percent = TIME_USAGE_PERCENT_DEFAULT;

        if self.moves_to_go == 0 {
            self.moves_to_go = ESTIMATED_REST_MOVES;
        }

        if g.board.side_to_move() == Color::White {
            info!("white's turn");
            security_move_time = self.white_time * 9 / 10;
            time_for_all_moves = self.white_time + (self.moves_to_go - 1) * self.white_inc;
        } else {
            info!("black's turn");
            security_move_time = self.white_time * 9 / 10;
            time_for_all_moves = self.black_time + (self.moves_to_go - 1) * self.black_inc;
        }
        info!("moves to go: {}", self.moves_to_go);
        info!("time for all moves: {}", time_for_all_moves);
        info!("move number: {}", g.move_number);

        //if (g.move_number < EARLY_GAME_MAX_MOVES) && (self.moves_to_go > EARLY_GAME_MIN_REST_MOVES)
        if g.move_number < EARLY_GAME_MAX_MOVES {
            time_usage_percent = TIME_USAGE_PERCENT_EARLY_GAME;
        }

        new_move_time = (time_for_all_moves * time_usage_percent) / (self.moves_to_go * 100);
        if new_move_time < MIN_MOVE_TIME {
            new_move_time = MIN_MOVE_TIME;
        }
        g.move_time = min(new_move_time, security_move_time);

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
