mod depth_search;
mod evaluator;
mod greedy;
mod mctree;
mod simple;
mod utils;

pub use self::depth_search::{DepthSearcher, RandomDepthSearcher};
pub use self::evaluator::*;
pub use self::greedy::GreedySearcher;
pub use self::mctree::McTreeSearcher;
pub use self::simple::{Interactive, RandomSearcher};
pub use utils::ab_search;

use std::fmt::Debug;

use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64 as Rng;

use crate::board::Board;

pub trait Searcher {
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

pub fn build_ai(stealing: bool, s: &str) -> Result<Box<dyn Searcher>, String> {
    let args = s.split(':').collect::<Vec<_>>();
    match args[0] {
        "human" => {
            if args.len() == 1 {
                return Ok(Box::new(Interactive::new(ScoreDiffEvaluator::new(), 0)));
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
                "diff" => Box::new(Interactive::new(ScoreDiffEvaluator::new(), max_depth)),
                "pos" => Box::new(Interactive::new(ScorePosEvaluator::new(), max_depth)),
                "nn4" => Box::new(Interactive::new(
                    NeuralNet4Evaluator::new(stealing),
                    max_depth,
                )),
                "nn6" => Box::new(Interactive::new(
                    NeuralNet6Evaluator::new(stealing),
                    max_depth,
                )),
                "mc" => {
                    if eval_args.len() != 2 {
                        return Err("human:mc-(num):(max_depth)".to_string());
                    }
                    let num = match eval_args[1].parse() {
                        Ok(d) => d,
                        Err(e) => return Err(format!("human:mc-(num):(max_depth) {}", e)),
                    };
                    Box::new(Interactive::new(
                        McTreeEvaluator::new(Rng::from_entropy(), num),
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
            Ok(Box::new(RandomSearcher::new(Rng::from_entropy())))
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
                "diff" => Box::new(DepthSearcher::new(ScoreDiffEvaluator::new(), max_depth)),
                "pos" => Box::new(DepthSearcher::new(ScorePosEvaluator::new(), max_depth)),
                "nn4" => Box::new(DepthSearcher::new(
                    NeuralNet4Evaluator::new(stealing),
                    max_depth,
                )),
                "nn6" => Box::new(DepthSearcher::new(
                    NeuralNet6Evaluator::new(stealing),
                    max_depth,
                )),
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
                "diff" => Box::new(RandomDepthSearcher::new(
                    max_depth,
                    weight,
                    ScoreDiffEvaluator::new(),
                    random,
                )),
                "pos" => Box::new(RandomDepthSearcher::new(
                    max_depth,
                    weight,
                    ScorePosEvaluator::new(),
                    random,
                )),
                "nn4" => Box::new(RandomDepthSearcher::new(
                    max_depth,
                    weight,
                    NeuralNet4Evaluator::new(stealing),
                    random,
                )),
                "nn6" => Box::new(RandomDepthSearcher::new(
                    max_depth,
                    weight,
                    NeuralNet6Evaluator::new(stealing),
                    random,
                )),
                _ => {
                    return Err("rdfs:(diff|pos|nn4|nn6):(max_depth):(weight)".to_string());
                }
            })
        }
        "mctree" => {
            if args.len() != 4 {
                return Err("mctree:{limit}:{ex}:{c}".to_owned());
            }
            let limit = args[1].parse::<u64>().map_err(|e| e.to_string())?;
            let ex = args[2].parse::<u32>().map_err(|e| e.to_string())?;
            let c = args[2].parse::<f64>().map_err(|e| e.to_string())?;
            Ok(Box::new(McTreeSearcher::new(
                Rng::from_entropy(),
                limit,
                ex,
                c,
            )))
        }
        "greedy" => {
            if args.len() != 1 {
                return Err("greedy".to_string());
            }
            Ok(Box::new(GreedySearcher::new(stealing, Rng::from_entropy())))
        }
        _ => Err("(human|random|dfs|rdfs|mctree|weighted|greedy)".to_string()),
    }
}
