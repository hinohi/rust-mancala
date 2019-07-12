mod depth_search;
mod leaf_learned;
mod mctree;
mod simple;
mod utils;

pub use self::depth_search::DepthSearchAI;
pub use self::leaf_learned::*;
pub use self::mctree::*;
pub use self::simple::{InteractiveAI, RandomAI};

pub trait AI {
    fn sow(&mut self, board: &crate::board::Board) -> Vec<usize>;
}
