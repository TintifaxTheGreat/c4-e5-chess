use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

use chess::{Board, ChessMove};

use super::game::{MoveValue, MoveValueOption};

pub struct Item {
    depth: u16,
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

    pub fn put(&mut self, depth: u16, value: i32, b: &Board, chessmove: &ChessMove) {
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
                    _ = &self.h.insert(key, item); //TODO check if we should use an update method
                }
            }
            Vacant(_) => {
                _ = &self.h.insert(key, item);
            }
        }
    }

    pub fn get(&mut self, depth: u16, b: &Board) -> Option<(ChessMove, i32)> {
        // TODO why do we have to use mutable? --> Change implementation!

        let key = *b;
        match &self.h.entry(key) {
            Occupied(val) => {
                let old_item = val.get();
                if old_item.depth < depth {
                    None
                } else {
                    Some((old_item.chessmove, old_item.value))
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
    use std::time::Duration;

    #[test]
    fn test_store() {
        let g = Game::new("".to_string(), 10, 10, Duration::new(10, 0));
        let mut store = Store::new();

        let result = store.get(5, &g.board);
        assert_eq!(result, None);

        store.put(
            5,
            300,
            &g.board,
            &ChessMove::from_san(&g.board, "c2c4").unwrap(),
        );

        let (m, v) = store.get(5, &g.board).unwrap();
        //assert_eq!(result, None);
        assert_eq!(v, 300);
        assert_eq!(m.to_string(), "c2c4");

        let result = store.get(6, &g.board);
        assert_eq!(result, None);

        let (m, v) = store.get(4, &g.board).unwrap();
        //assert_eq!(result, None);
        assert_eq!(v, 300);
        assert_eq!(m.to_string(), "c2c4");

        store.put(
            5,
            305,
            &g.board,
            &ChessMove::from_san(&g.board, "e2e4").unwrap(),
        );

        let (m, v) = store.get(4, &g.board).unwrap();
        //assert_eq!(result, None);
        assert_eq!(v, 305);
        assert_eq!(m.to_string(), "e2e4")
    }
}
