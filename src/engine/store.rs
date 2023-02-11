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
