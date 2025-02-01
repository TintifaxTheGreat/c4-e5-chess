use crate::misc::types::*;
use cozy_chess::{Board, Move};
use hashbrown::hash_map::Entry::Occupied;
use hashbrown::hash_map::Entry::Vacant;
use hashbrown::HashMap;

/// A transposition table.
#[derive(Clone)]
pub struct Item {
    depth: Depth,
    value: MoveScore,
    chessmove: Move,
}

/// A hashmap for use in the transposition table.
pub struct Store {
    pub h: HashMap<u64, Item>,
}

impl Store {
    /// Constructor
    pub fn new() -> Self {
        Self { h: HashMap::new() }
    }

    /// Put a position, its score and depth and the best move into the transposition table.
    /// Update the score only if depth is greater than already stored depth.
    pub fn put(&mut self, depth: Depth, value: MoveScore, b: &Board, chessmove: &Move) {
        let key = b.hash_without_ep();
        let item = Item {
            depth,
            value,
            chessmove: *chessmove,
        };
        match self.h.entry(key) {
            Occupied(mut entry) => {
                if entry.get().depth <= depth {
                    entry.insert(item);
                }
            }
            Vacant(entry) => {
                entry.insert(item);
            }
        }
    }

    /// Get a move and its score for the given position.
    pub fn get(&self, depth: Depth, b: &Board) -> Option<(Move, MoveScore, bool)> {
        let key = b.hash_without_ep();
        match self.h.get(&key) {
            Some(item) => {
                if item.depth < depth {
                    Some((item.chessmove, item.value, false))
                } else {
                    Some((item.chessmove, item.value, true))
                }
            }
            None => None,
        }
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::engine::game::Game;

    #[test]
    fn test_store() {
        let g = Game::new("".to_string(), 10, 10000);
        let mut store = Store::new();

        let result = store.get(5, &g.board);
        assert_eq!(result, None);

        store.put(5, 300, &g.board, &Move::from_str("c2c4").unwrap());

        let (m, v, fresh) = store.get(5, &g.board).unwrap();
        assert_eq!(v, 300);
        assert_eq!(m.to_string(), "c2c4");
        assert_eq!(fresh, true);

        let (m, _, fresh) = store.get(6, &g.board).unwrap();
        assert_eq!(m.to_string(), "c2c4");
        assert_eq!(fresh, false);

        let (m, v, fresh) = store.get(4, &g.board).unwrap();
        assert_eq!(v, 300);
        assert_eq!(m.to_string(), "c2c4");
        assert_eq!(fresh, true);

        store.put(5, 305, &g.board, &Move::from_str("e2e4").unwrap());

        let (m, v, fresh) = store.get(4, &g.board).unwrap();
        assert_eq!(v, 305);
        assert_eq!(m.to_string(), "e2e4");
        assert_eq!(fresh, true);
    }
}
