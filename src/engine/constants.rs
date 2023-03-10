// Bitmaps

use super::game::{Depth, MoveTime, MoveScore};

pub const CB_CENTER: u64 = 0x00003C3C3C3C0000;
pub const CB_BOARD_0: u64 = 0xff818181818181ff;
pub const CB_BOARD_1: u64 = 0x007e424242427e00;
pub const CB_CENTER_0: u64 = 0x00003c24243c0000;
pub const CB_CENTER_1: u64 = 0x0000001818000000;
pub const CB_SAFE_KING: u64 = 0xc3000000000000c3;
pub const CB_GOOD_BISHOP: u64 = 0x42006666004200;
pub const CB_GOOD_QUEEN: u64 = 0x3c1800000000183c;
pub const CB_BASE_LINE: u64 = 0xff000000000000ff;

// FEN
pub const FEN_START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Search
pub const MAX_INT: MoveScore = 1_000_000;
pub const MIN_INT: MoveScore = -1_000_000;
pub const INIT_MAX_DEPTH: Depth = 99;
pub const INIT_QUIET_DEPTH: Depth = 1;
pub const LATE_PRUNING_PERCENT: MoveScore = 85;
pub const LATE_PRUNING_INDEX: usize = 2;
pub const LATE_PRUNING_DEPTH_START: Depth = 4;
pub const LATE_MOVE_REDUCTION_DEPTH_START: Depth = 4;
pub const LATE_PRUNING_DEPTH_REDUCTION: Depth = 2;
pub const PVS_DEPTH: Depth = 2;

// Game
pub const DEFAULT_TIME: MoveTime = 10_000; // in Milliseconds

// Evaluation
pub const MATE_LEVEL: MoveScore = 55_000;
pub const MATE: MoveScore = 60_000;
