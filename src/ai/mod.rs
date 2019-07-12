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

pub fn build_ai(s: &str) -> Result<Box<AI>, String> {
    use crate::board::ScoreDiffEvaluation;
    use rand::SeedableRng;
    use rand_pcg::Mcg128Xsl64 as Rng;

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
                ScoreDiffEvaluation::new(),
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
        _ => Err("(human|random|dfs|mctree)".to_string()),
    }
}
