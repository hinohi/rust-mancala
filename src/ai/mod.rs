mod base;
mod depth_search;
mod simple;

pub use self::base::{Judge, AI};
pub use self::depth_search::DepthSearchAI;
pub use self::simple::{InteractiveAI, RandomAI};
