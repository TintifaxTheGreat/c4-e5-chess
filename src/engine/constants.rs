use std::time::Duration;

// Bitmaps

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
pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Search
pub const MAX_INT: i32 = 1000000;
pub const MIN_INT: i32 = -1000000;
pub const INIT_MAX_DEPTH: u16 = 99;
pub const INIT_QUIET_DEPTH: u16 = 1;
pub const LATE_PRUNING_THRESHOLD: u16 = 60;
pub const LATE_PRUNING_DEPTH: u16 = 3;
pub const PVS_DEPTH: u32 = 2;

// Game
pub const DEFAULT_TIME: Duration =  Duration::from_millis(10000);

// Evaluation
pub const MATE_LEVEL: i32 = 55000;
pub const MATE: i32 = 60000;
