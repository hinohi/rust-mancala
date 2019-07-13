use super::Evaluator;
use crate::board::Board;

#[derive(Debug)]
pub struct ScoreDiffEvaluator;

impl ScoreDiffEvaluator {
    pub fn new() -> ScoreDiffEvaluator {
        ScoreDiffEvaluator {}
    }
}

impl Evaluator for ScoreDiffEvaluator {
    fn eval(&self, board: &Board) -> i32 {
        let (s0, s1) = board.last_scores();
        if board.side == 0 {
            s0 as i32 - s1 as i32
        } else {
            s1 as i32 - s0 as i32
        }
    }
}
