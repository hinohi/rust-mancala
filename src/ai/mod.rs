mod depth_search;
mod evaluator;
mod greedy;
mod mctree;
mod simple;
mod utils;

pub use self::depth_search::DepthSearchAI;
pub use self::evaluator::*;
pub use self::greedy::GreedyAI;
pub use self::mctree::MCTree;
pub use self::simple::{InteractiveAI, RandomAI};

use std::fmt::Debug;

use rand::SeedableRng;
use rand_pcg::{Mcg128Xsl64 as Rng, Mcg128Xsl64};

use crate::ai::mctree::WeightedMCTree;
use crate::board::Board;

pub trait AI {
    fn sow(&mut self, board: &Board) -> Vec<usize>;
}

pub trait Evaluator {
    type Score: Score;
    fn eval(&mut self, board: &Board) -> Self::Score;
}

pub trait Score: PartialOrd + Copy + Debug {
    const MIN: Self;
    const MAX: Self;
    fn flip(&self) -> Self;
}

pub fn build_ai(stealing: bool, s: &str) -> Result<Box<dyn AI>, String> {
    let args = s.split(':').collect::<Vec<_>>();
    match args[0] {
        "human" => {
            if args.len() == 1 {
                return Ok(Box::new(InteractiveAI::new(ScoreDiffEvaluator::new(), 0)));
            }
            if args.len() != 3 {
                return Err("human[:(eval):(max_depth)]".to_string());
            }
            let max_depth = match args[2].parse() {
                Ok(d) => d,
                Err(e) => return Err(format!("human[:(eval):(max_depth)] {}", e)),
            };
            let eval_args = args[1].split('-').collect::<Vec<_>>();
            Ok(match eval_args[0] {
                "diff" => Box::new(InteractiveAI::new(ScoreDiffEvaluator::new(), max_depth)),
                "pos" => Box::new(InteractiveAI::new(ScorePosEvaluator::new(), max_depth)),
                "nn4" => Box::new(InteractiveAI::new(NN4Evaluator::new(stealing), max_depth)),
                "nn6" => Box::new(InteractiveAI::new(NN6Evaluator::new(stealing), max_depth)),
                "mc" => {
                    if eval_args.len() != 2 {
                        return Err("human:mc-(num):(max_depth)".to_string());
                    }
                    let num = match eval_args[1].parse() {
                        Ok(d) => d,
                        Err(e) => return Err(format!("human:mc-(num):(max_depth) {}", e)),
                    };
                    Box::new(InteractiveAI::new(
                        MCTreeEvaluator::new(Rng::from_entropy(), num),
                        max_depth,
                    ))
                }
                _ => {
                    return Err("human[:(diff|pos|nn4|nn6|mc-(num)):(max_depth)]".to_string());
                }
            })
        }
        "random" => {
            if args.len() != 1 {
                return Err("random".to_string());
            }
            Ok(Box::new(RandomAI::new(Rng::from_entropy())))
        }
        "dfs" => {
            if args.len() != 3 {
                return Err("dfs:(eval):(max_depth)".to_string());
            }
            let max_depth = match args[2].parse() {
                Ok(d) => d,
                Err(e) => return Err(format!("dfs:(eval):(max_depth) {}", e)),
            };
            let eval_args = args[1].split('-').collect::<Vec<_>>();
            Ok(match eval_args[0] {
                "diff" => Box::new(DepthSearchAI::new(ScoreDiffEvaluator::new(), max_depth)),
                "pos" => Box::new(DepthSearchAI::new(ScorePosEvaluator::new(), max_depth)),
                "nn4" => Box::new(DepthSearchAI::new(NN4Evaluator::new(stealing), max_depth)),
                "nn6" => Box::new(DepthSearchAI::new(NN6Evaluator::new(stealing), max_depth)),
                "mc" => {
                    if eval_args.len() != 2 {
                        return Err("dfs:mc-(num):(max_depth)".to_string());
                    }
                    let num = match eval_args[1].parse() {
                        Ok(d) => d,
                        Err(e) => return Err(format!("dfs:mc-(num):(max_depth) {}", e)),
                    };
                    Box::new(DepthSearchAI::new(
                        MCTreeEvaluator::new(Rng::from_entropy(), num),
                        max_depth,
                    ))
                }
                _ => {
                    return Err("dfs:(diff|pos|nn4|nn6|mc-(num)):(max_depth)".to_string());
                }
            })
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
        "weighted" => {
            if args.len() != 3 {
                return Err("weighted:(eval):(num)".to_string());
            }
            let num = match args[2].parse::<u32>() {
                Ok(e) => 1_u32 << e,
                Err(e) => return Err(format!("mctree:(num) {}", e)),
            } as usize;
            Ok(match args[1] {
                "diff" => Box::new(WeightedMCTree::new(
                    num,
                    Rng::from_entropy(),
                    ScoreDiffEvaluator::new(),
                )),
                "pos" => Box::new(WeightedMCTree::new(
                    num,
                    Rng::from_entropy(),
                    ScorePosEvaluator::new(),
                )),
                "nn4" => Box::new(WeightedMCTree::new(
                    num,
                    Rng::from_entropy(),
                    NN4Evaluator::new(stealing),
                )),
                "nn6" => Box::new(WeightedMCTree::new(
                    num,
                    Rng::from_entropy(),
                    NN6Evaluator::new(stealing),
                )),
                _ => {
                    return Err("weighted:(diff|pos|nn4|nn6):(num)".to_string());
                }
            })
        }
        "greedy" => {
            if args.len() != 1 {
                return Err("greedy".to_string());
            }
            Ok(Box::new(GreedyAI::new(
                stealing,
                Mcg128Xsl64::from_entropy(),
            )))
        }
        _ => Err("(human|random|dfs|mctree|weighted|sparse|greedy)".to_string()),
    }
}
