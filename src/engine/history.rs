use super::types::*;
use chess::Board;
use hashbrown::hash_map::Entry::{Occupied, Vacant};
use hashbrown::HashMap;

pub struct History {
    h: HashMap<u64, BoardHistory>,
}

impl History {
    pub fn new() -> Self {
        Self { h: HashMap::new() }
    }

    pub fn inc(&mut self, b: &Board) {
        let key = b.get_hash();
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

    pub fn dec(&mut self, b: &Board) {
        let key = b.get_hash();
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

    pub fn get(&mut self, b: &Board) -> BoardHistory {
        let key = b.get_hash();
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
