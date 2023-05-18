//! `C4-E5 Chess` is a UCI compatible chess engine based on the move generator in crate Chess.
//!
//! These features are provided:
//! * Parallelised iterative depthening
//! * Late move pruning
//! * Principal variant search
//! * Transposition table

/// UCI connector
pub mod cmd;

/// Chess engine
pub mod engine;

/// Helpers
pub mod misc;
