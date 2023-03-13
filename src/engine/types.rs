use chess::ChessMove;

pub type Depth = i16;
pub type MoveTime = u64;
pub type MoveNumber = u64;
pub type MoveScore = i32;

pub struct ScoredMove {
    pub mv: ChessMove,
    pub sc: MoveScore,
    pub incr: bool,
}

#[derive(Clone, Copy)]
pub struct AnnotatedMove {
    pub mv: ChessMove,
    pub capture: bool,
}