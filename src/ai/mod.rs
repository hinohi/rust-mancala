mod depth_search;
mod evaluator;
mod greedy;
mod mctree;
mod simple;
mod utils;

pub use self::depth_search::{DepthSearchAI, RandomDepthSearchAI};
pub use self::evaluator::*;
pub use self::greedy::GreedyAI;
pub use self::mctree::McTreeAI;
pub use self::simple::{InteractiveAI, RandomAI};
pub use utils::ab_search;

use std::fmt::Debug;

use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64 as Rng;

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
                _ => {
                    return Err("dfs:(diff|pos|nn4|nn6|mc-(num)):(max_depth)".to_string());
                }
            })
        }
        "rdfs" => {
            if args.len() != 4 {
                return Err("rdfs:(eval):(max_depth):(weight)".to_string());
            }
            let max_depth = match args[2].parse() {
                Ok(d) => d,
                Err(e) => return Err(format!("max_depth must be usize: {}", e)),
            };
            let weight = match args[3].parse() {
                Ok(w) => w,
                Err(e) => return Err(format!("weight must be f64: {}", e)),
            };
            let random = Rng::from_entropy();
            Ok(match args[1] {
                "diff" => Box::new(RandomDepthSearchAI::new(
                    max_depth,
                    weight,
                    ScoreDiffEvaluator::new(),
                    random,
                )),
                "pos" => Box::new(RandomDepthSearchAI::new(
                    max_depth,
                    weight,
                    ScorePosEvaluator::new(),
                    random,
                )),
                "nn4" => Box::new(RandomDepthSearchAI::new(
                    max_depth,
                    weight,
                    NN4Evaluator::new(stealing),
                    random,
                )),
                "nn6" => Box::new(RandomDepthSearchAI::new(
                    max_depth,
                    weight,
                    NN6Evaluator::new(stealing),
                    random,
                )),
                _ => {
                    return Err("rdfs:(diff|pos|nn4|nn6):(max_depth):(weight)".to_string());
                }
            })
        }
        "mctree" => Ok(Box::new(McTreeAI::new(Rng::from_entropy()))),
        "greedy" => {
            if args.len() != 1 {
                return Err("greedy".to_string());
            }
            Ok(Box::new(GreedyAI::new(stealing, Rng::from_entropy())))
        }
        _ => Err("(human|random|dfs|rdfs|mctree|weighted|greedy)".to_string()),
    }
}
