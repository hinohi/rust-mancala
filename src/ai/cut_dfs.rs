use std::i32;

use super::base::AI;
use crate::game::{Board, Evaluation};

pub struct CutDepthAI<E> {
    max_width: usize,
    max_depth: usize,
    evaluator: E,
}

impl<E> CutDepthAI<E>
where
    E: Evaluation,
{
    pub fn new(evaluator: E, max_width: usize, max_depth: usize) -> CutDepthAI<E> {
        CutDepthAI {
            max_width,
            max_depth,
            evaluator,
        }
    }

    fn search(&self, board: Board, depth: usize) -> i32 {
        if depth == 0 || board.is_finished() {
            return self.evaluator.eval(&board);
        }
        let mut best = i32::MIN;
        let mut next_list = board.list_next().drain().collect::<Vec<_>>();
        next_list.sort_by_key(|board| -self.evaluator.eval(board));
        for next in next_list.into_iter().take(self.max_width) {
            let s = -self.search(next, depth - 1);
            if s > best {
                best = s;
            }
        }
        best
    }
}

impl<E> AI for CutDepthAI<E>
where
    E: Evaluation,
{
    fn think(&mut self, board: &Board) -> Vec<usize> {
        let mut next_lists = board.list_next_with_pos();
        if next_lists.len() == 1 {
            return next_lists.drain().next().unwrap().1;
        }
        let mut best = vec![];
        let mut best_score = i32::MIN;
        for (next, pos_list) in next_lists {
            let s = -self.search(next, self.max_depth);
            if s > best_score {
                best_score = s;
                best = pos_list;
            }
        }
        best
    }
}
