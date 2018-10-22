mod base;
mod depth_search;
mod mem;
mod simple;

pub use self::base::{Judge, AI};
pub use self::depth_search::DepthSearchAI;
pub use self::mem::MemAI;
pub use self::simple::{InteractiveAI, RandomAI};
