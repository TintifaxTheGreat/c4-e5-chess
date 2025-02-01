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

// Evaluation
/// Score above which a game is considered as won
pub const MATE_LEVEL: MoveScore = 55_000;
/// Score for mate
pub const MATE: MoveScore = 60_000;
