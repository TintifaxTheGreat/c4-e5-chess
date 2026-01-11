use crate::misc::types::*;

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

/// Default number of threads
pub const DEFAULT_NUM_THREADS: usize = 1;

/// Minimal number of threads
pub const MIN_NUM_THREADS: usize = 1;

/// Maximal number of threads
pub const MAX_NUM_THREADS: usize = 16;

/// Default hash size in MB
pub const DEFAULT_HASH_SIZE: usize = 16;

/// Minimal hash size in MB
pub const MIN_HASH_SIZE: usize = 1;

/// Maximal hash size in MB
pub const MAX_HASH_SIZE: usize = 1024;

// Evaluation
/// Score above which a game is considered as won
pub const MATE_LEVEL: MoveScore = 55_000;
/// Score for mate
pub const MATE: MoveScore = 60_000;
