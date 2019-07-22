mod ai;
mod board;
mod game;
pub mod learn;

pub use ai::*;
pub use board::{compact_key, from_compact_key, Board, Side, PIT, SEED};
pub use game::Game;

#[macro_use]
extern crate lazy_static;
