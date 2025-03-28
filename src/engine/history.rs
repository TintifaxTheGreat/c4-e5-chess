use crate::misc::types::*;
use cozy_chess::Board;
use hashbrown::hash_map::Entry::{Occupied, Vacant};
use hashbrown::HashMap;

/// A counter how often a position has already occured.
pub struct History {
    pub h: HashMap<u64, BoardHistory>,
}

impl History {
    /// Constructor
    pub fn new() -> Self {
        Self { h: HashMap::new() }
    }

    /// Increment counter for given position.
    pub fn inc(&mut self, b: &Board) {
        let key = b.hash_without_ep();
        match &self.h.entry(key) {
            Occupied(val) => {
                let new_value = val.get() + 1;
                _ = &self.h.insert(key, new_value);
            }
            Vacant(_) => {
                _ = &self.h.insert(key, 1);
            }
        }
    }

    /// Decrement counter for given position.
    pub fn dec(&mut self, b: &Board) {
        let key = b.hash_without_ep();
        match &self.h.entry(key) {
            Occupied(val) => {
                let new_value = val.get() - 1;
                _ = &self.h.insert(key, new_value);
            }
            Vacant(_) => {
                _ = &self.h.insert(key, 0);
            }
        }
    }

    /// Get counter for the given position.
    pub fn get(&mut self, b: &Board) -> BoardHistory {
        let key = b.hash_without_ep();
        match &self.h.entry(key) {
            Occupied(val) => *val.get(),
            Vacant(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::game::Game;

    #[test]
    fn test_history() {
        let g1 = Game::new("".to_string(), 10, 10000);
        let g2 = Game::new(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string(),
            10,
            10000,
        );
        let mut history = History::new();

        let result = history.get(&g1.board);
        assert_eq!(result, 0);

        history.inc(&g1.board);

        let result = history.get(&g1.board);
        assert_eq!(result, 1);

        let result = history.get(&g2.board);
        assert_eq!(result, 0);

        history.inc(&g1.board);
        history.inc(&g1.board);

        let result = history.get(&g1.board);
        assert_eq!(result, 3);

        history.dec(&g1.board);

        let result = history.get(&g1.board);
        assert_eq!(result, 2);
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
