use rand::Rng;

use super::evaluator::{MCTreeEvaluator, WeightedMCTreeEvaluator, WinRateScore};
use super::{Evaluator, Score, AI};
use crate::board::Board;

#[derive(Debug, Clone)]
pub struct MCTree<R: Rng> {
    path_num: usize,
    evaluator: MCTreeEvaluator<R>,
}

impl<R> MCTree<R>
where
    R: Rng,
{
    pub fn new(path_num: usize, random: R) -> MCTree<R> {
        MCTree {
            path_num,
            evaluator: MCTreeEvaluator::new(random, 0),
        }
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
        self.evaluator.set_num((self.path_num + n - 1) / n);
        // `MCTreeEvaluator::Score::MIN` だと `ambiguous associated type` と怒られる
        let mut best = WinRateScore::MIN;
        let mut best_pos = vec![];
        for (board, pos) in next_lists {
            let score = self.evaluator.eval(&board).flip();
            if best < score {
                best = score;
                best_pos = pos;
            }
        }
        best_pos
    }
}

#[derive(Debug, Clone)]
pub struct WeightedMCTree<R, E> {
    path_num: usize,
    evaluator: WeightedMCTreeEvaluator<R, E>,
}

impl<R, E> WeightedMCTree<R, E>
where
    R: Rng,
    E: Evaluator,
    E::Score: Into<f64>,
{
    pub fn new(path_num: usize, random: R, eval: E) -> WeightedMCTree<R, E> {
        WeightedMCTree {
            path_num,
            evaluator: WeightedMCTreeEvaluator::new(random, eval, 0),
        }
    }
}

impl<R, E> AI for WeightedMCTree<R, E>
where
    R: Rng,
    E: Evaluator,
    E::Score: Into<f64>,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let mut next_lists = board.list_next_with_pos();
        let n = next_lists.len();
        if n == 1 {
            return next_lists.drain().next().unwrap().1;
        }
        self.evaluator.set_num((self.path_num + n - 1) / n);
        let mut best = WinRateScore::MIN;
        let mut best_pos = vec![];
        for (board, pos) in next_lists {
            let score = self.evaluator.eval(&board).flip();
            if best < score {
                best = score;
                best_pos = pos;
            }
        }
        best_pos
    }
}
