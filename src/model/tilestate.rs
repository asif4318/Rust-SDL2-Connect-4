use std::fmt::{Display, Formatter};

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum TileState {
    EMPTY = 0,
    YELLOW = 1,
    RED = 2,
}

impl Display for TileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileState::EMPTY => f.write_str("EMPTY"),
            TileState::RED => f.write_str("RED"),
            TileState::YELLOW => f.write_str("YELLOW"),
        }
    }
}