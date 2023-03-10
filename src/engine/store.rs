use chess::{Board, ChessMove};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

pub struct Item {
    depth: i16,
    value: i32,
    chessmove: ChessMove,
}

pub struct Store {
    h: HashMap<Board, Item>,
}

impl Store {
    pub fn new() -> Self {
        Self { h: HashMap::new() }
    }

    pub fn put(&mut self, depth: i16, value: i32, b: &Board, chessmove: &ChessMove) {
        let key = *b;
        let item = Item {
            depth,
            value,
            chessmove: *chessmove,
        };
        match &self.h.entry(key) {
            Occupied(val) => {
                let old_item = val.get();
                if old_item.depth <= depth {
                    _ = &self.h.insert(key, item);
                }
            }
            Vacant(_) => {
                _ = &self.h.insert(key, item);
            }
        }
    }

    pub fn get(&mut self, depth: i16, b: &Board) -> Option<(ChessMove, i32, bool)> {
        // TODO why do we have to use mutable? --> Change implementation!

        let key = *b;
        match &self.h.entry(key) {
            Occupied(val) => {
                let old_item = val.get();
                if old_item.depth < depth {
                    Some((old_item.chessmove, old_item.value, false))
                } else {
                    Some((old_item.chessmove, old_item.value, true))
                }
            }
            Vacant(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::game::Game;

    #[test]
    fn test_store() {
        let g = Game::new("".to_string(), 10, 10000);
        let mut store = Store::new();

        let result = store.get(5, &g.board);
        assert_eq!(result, None);

        store.put(
            5,
            300,
            &g.board,
            &ChessMove::from_san(&g.board, "c2c4").unwrap(),
        );

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

        store.put(
            5,
            305,
            &g.board,
            &ChessMove::from_san(&g.board, "e2e4").unwrap(),
        );

        let (m, v, fresh) = store.get(4, &g.board).unwrap();
        assert_eq!(v, 305);
        assert_eq!(m.to_string(), "e2e4");
        assert_eq!(fresh, true);
    }
}
