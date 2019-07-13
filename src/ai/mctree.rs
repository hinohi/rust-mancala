use rand::Rng;

use super::evaluator::{MCTreeEvaluator, WinRateScore};
use super::{Evaluator, Score, AI};
use crate::board::Board;

#[derive(Debug)]
pub struct MCTree<R: Rng> {
    path_num: usize,
    random: R,
}

impl<R> MCTree<R>
where
    R: Rng,
{
    pub fn new(path_num: usize, random: R) -> MCTree<R> {
        MCTree { path_num, random }
    }
}

impl<R> AI for MCTree<R>
where
    R: Rng,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let mut next_lists = board.list_next_with_pos();
        let n = next_lists.len();
        if n == 1 {
            return next_lists.drain().next().unwrap().1;
        }
        let mut eval = MCTreeEvaluator::new(&mut self.random, (self.path_num + n - 1) / n);
        // `MCTreeEvaluator::Score::MIN` だと `ambiguous associated type` と怒られる
        let mut best = WinRateScore::MIN;
        let mut best_pos = vec![];
        for (board, pos) in next_lists {
            let score = eval.eval(&board).flip();
            if best < score {
                best = score;
                best_pos = pos;
            }
        }
        best_pos
    }
}
