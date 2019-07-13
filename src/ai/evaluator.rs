use super::Evaluator;
use crate::board::Board;

#[derive(Debug, Default)]
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
            i32::from(s0) - i32::from(s1)
        } else {
            i32::from(s1) - i32::from(s0)
        }
    }
}
