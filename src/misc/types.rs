use cozy_chess::Move;

pub type Depth = i16;
pub type MoveTime = u64;
pub type MoveNumber = u64;
pub type MoveScore = i32;
pub type BoardHistory = u16;

/// A chess move and its score including a capture flag.
#[derive(Clone, Copy)]
pub struct AnnotatedMove {
    pub mv: Move,
    pub sc: MoveScore,
    pub cp: bool,
    pub node_count: u64,
}
