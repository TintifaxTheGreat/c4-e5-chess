use super::types::*;

// Bitmaps
/// Center of the board
pub const CB_CENTER: u64 = 0x00003C3C3C3C0000;

/// Inner center of the board
pub const CB_CENTER_0: u64 = 0x00003c24243c0000;

/// Outer center of the board
pub const CB_CENTER_1: u64 = 0x0000001818000000;

/// Outer edge of the board
pub const CB_BOARD_0: u64 = 0xff818181818181ff;

/// Inner edge of the board
pub const CB_BOARD_1: u64 = 0x007e424242427e00;

/// Best squares for center pawns
pub const CB_CENTER_0_GOOD: u64 = 0x182424180000;

/// Best squares for king's safety
pub const CB_SAFE_KING: u64 = 0xc3000000000000c3;

/// Best squares for bishop
pub const CB_GOOD_BISHOP: u64 = 0x42006666004200;

/// Best squares for queen
pub const CB_GOOD_QUEEN: u64 = 0x3c1800000000183c;

/// 1st rank
pub const CB_RANK_1: u64 = 0xff;

/// 2nd rank
pub const CB_RANK_2: u64 = 0xff00;

/// 3rd rank
pub const CB_RANK_3: u64 = 0xff0000;

/// 6th rank
pub const CB_RANK_6: u64 = 0xff0000000000;

/// 7th rank
pub const CB_RANK_7: u64 = 0xff000000000000;

/// 8th rank
pub const CB_RANK_8: u64 = 0xff00000000000000;

// FEN
/// Start position encoded as FEN
pub const FEN_START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Search

/// Maximum score
pub const MAX_INT: MoveScore = 1_000_000;

/// Minimum score
pub const MIN_INT: MoveScore = -1_000_000;

/// Maximal search depth
pub const INIT_MAX_DEPTH: Depth = 99;

/// Depth to start forward pruning
pub const FORWARD_PRUNING_DEPTH_START: Depth = 4;

/// Minimum number of moves after application of forward pruning
pub const FORWARD_PRUNING_MINIMUM: usize = 4;

/// Ratio w.r.t. score for moves to keep during forward pruning
pub const FORWARD_PRUNING_RATIO: usize = 4;

// Game
/// Default time for one move
pub const DEFAULT_TIME: MoveTime = 10_000; // in Milliseconds

// Evaluation
/// Score above which a game is considered as won
pub const MATE_LEVEL: MoveScore = 55_000;
/// Score for mate
pub const MATE: MoveScore = 60_000;
