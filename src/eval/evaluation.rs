use crate::misc::types::*;
use cozy_chess::Board;

/// A board evaluation
pub trait Evaluation {
    /// An evaluation function
    fn evaluate(board: &Board) -> MoveScore;
}
