use crate::engine::types::*;

// Time Management
pub const MIN_MOVE_TIME: MoveTime = 200;
pub const MOVE_TIME_FRACTION_EARLY_GAME: u64 = 12;
pub const MOVE_TIME_FRACTION_LATE_GAME: u64 = 30;
pub const MOVE_LATE_GAME_START: MoveNumber = 20;
