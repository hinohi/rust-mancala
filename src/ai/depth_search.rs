use super::{utils::ab_search, Evaluator, Score, AI};
use crate::board::Board;

#[derive(Debug, Clone)]
pub struct DepthSearchAI<E> {
    max_depth: usize,
    evaluator: E,
}

impl<E> DepthSearchAI<E>
where
    E: Evaluator,
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
    E: Evaluator,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let mut next_lists = board.list_next_with_pos();
        if next_lists.len() == 1 {
            return next_lists.drain().next().unwrap().1;
        }
        let mut best = vec![];
        let mut best_score = E::Score::MIN;
        for (next, pos_list) in next_lists {
            let s = ab_search(
                next,
                &mut self.evaluator,
                self.max_depth,
                E::Score::MIN,
                E::Score::MAX,
            )
            .flip();
            if s > best_score {
                best_score = s;
                best = pos_list;
            }
        }
        best
    }
}
