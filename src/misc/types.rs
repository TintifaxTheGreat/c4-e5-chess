use chess::ChessMove;

pub type Depth = i16;
pub type MoveTime = u64;
pub type MoveNumber = u64;
pub type MoveScore = i32;
pub type BoardHistory = u16;

/// A chess move and its score including a capture flag.
#[derive(Clone, Copy)]
pub struct AnnotatedMove {
    pub mv: ChessMove,
    pub sc: MoveScore,
    pub node_count: u64,
    pub capture: bool,
}
