use std::i32;

use super::base::AI;
use super::utils::ab_search;
use crate::game::{Board, Evaluation};

pub struct DepthSearchAI<E> {
    max_depth: usize,
    evaluator: E,
}

impl<E> DepthSearchAI<E>
where
    E: Evaluation,
{
    pub fn new(evaluator: E, max_depth: usize) -> DepthSearchAI<E> {
        DepthSearchAI {
            max_depth,
            evaluator,
        }
    }
}

impl<E> AI for DepthSearchAI<E>
where
    E: Evaluation,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let mut next_lists = board.list_next_with_pos();
        if next_lists.len() == 1 {
            return next_lists.drain().next().unwrap().1;
        }
        let mut best = vec![];
        let mut best_score = i32::MIN;
        for (next, pos_list) in next_lists {
            let s = -ab_search(next, &self.evaluator, self.max_depth, -10000, 10000);
            if s > best_score {
                best_score = s;
                best = pos_list;
            }
        }
        best
    }
}
