use crate::misc::types::*;
use chess::Board;

/// A board evaluation
pub trait Evaluation {
    /// An evaluation function
    fn evaluate(board: &Board) -> MoveScore;
}
