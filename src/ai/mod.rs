mod depth_search;
mod evaluator;
mod leaf_learned;
mod mctree;
mod simple;
mod sparse;
mod utils;

pub use self::depth_search::DepthSearchAI;
pub use self::evaluator::*;
pub use self::leaf_learned::*;
pub use self::mctree::MCTree;
pub use self::simple::{InteractiveAI, RandomAI};
pub use self::sparse::SparseDepthSearchAI;

use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64 as Rng;

use crate::board::Board;

pub trait AI {
    fn sow(&mut self, board: &Board) -> Vec<usize>;
}

pub trait Evaluator {
    type Score: Score;
    fn eval(&self, board: &Board) -> Self::Score;
}

pub trait Score: Ord + Copy {
    const MIN: Self;
    const MAX: Self;
    fn flip(&self) -> Self;
}

pub fn build_ai(s: &str) -> Result<Box<AI>, String> {
    let args = s.split(':').collect::<Vec<_>>();
    match args[0] {
        "human" => {
            if args.len() != 1 {
                return Err("human".to_string());
            }
            Ok(Box::new(InteractiveAI::new()))
        }
        "random" => {
            if args.len() != 1 {
                return Err("random".to_string());
            }
            Ok(Box::new(RandomAI::new(Rng::from_entropy())))
        }
        "dfs" => {
            if args.len() != 2 {
                return Err("dfs:(max_depth)".to_string());
            }
            let max_depth = match args[1].parse() {
                Ok(d) => d,
                Err(e) => return Err(format!("dfs:(max_depth) {}", e)),
            };
            Ok(Box::new(DepthSearchAI::new(
                ScoreDiffEvaluator::new(),
                max_depth,
            )))
        }
        "mctree" => {
            if args.len() != 2 {
                return Err("mctree:(num)".to_string());
            }
            let num = match args[1].parse::<u32>() {
                Ok(e) => 1_u32 << e,
                Err(e) => return Err(format!("mctree:(num) {}", e)),
            };
            Ok(Box::new(MCTree::new(num as usize, Rng::from_entropy())))
        }
        "sparse" => {
            if args.len() != 3 {
                return Err("sparse:(first):(num)".to_string());
            }
            let first_depth = match args[1].parse() {
                Ok(d) => d,
                Err(e) => return Err(format!("sparse:(first):(num) {}", e)),
            };
            let num = match args[2].parse() {
                Ok(n) => n,
                Err(e) => return Err(format!("sparse:(first):(num) {}", e)),
            };
            Ok(Box::new(SparseDepthSearchAI::new(
                first_depth,
                num,
                Rng::from_entropy(),
            )))
        }
        _ => Err("(human|random|dfs|mctree|sparse)".to_string()),
    }
}
