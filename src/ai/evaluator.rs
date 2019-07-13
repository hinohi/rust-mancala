use super::{Evaluator, Score};
use crate::board::Board;

impl Score for i32 {
    const MIN: Self = std::i32::MIN;
    const MAX: Self = std::i32::MAX;
    #[inline]
    fn flip(&self) -> Self {
        -*self
    }
}

impl Score for i8 {
    const MIN: Self = std::i8::MIN;
    const MAX: Self = std::i8::MAX;
    #[inline]
    fn flip(&self) -> Self {
        -*self
    }
}

#[derive(Debug, Default)]
pub struct ScoreDiffEvaluator;

impl ScoreDiffEvaluator {
    pub fn new() -> ScoreDiffEvaluator {
        ScoreDiffEvaluator {}
    }
}

impl Evaluator for ScoreDiffEvaluator {
    type Score = i32;
    fn eval(&self, board: &Board) -> i32 {
        let (s0, s1) = board.last_scores();
        if board.side == 0 {
            i32::from(s0) - i32::from(s1)
        } else {
            i32::from(s1) - i32::from(s0)
        }
    }
}
