use crate::misc::types::*;

// Time Management
/// Minimum move time for any move in milliseconds
pub const MIN_MOVE_TIME: MoveTime = 200;

/// Fraction of time to be used for early game
pub const MOVE_TIME_FRACTION_EARLY_GAME: u64 = 12;

/// Fraction of time to be used for late game
pub const MOVE_TIME_FRACTION_LATE_GAME: u64 = 30;

/// Start move for late game
pub const MOVE_LATE_GAME_START: MoveNumber = 20;
