use std::i32;

use super::base::AI;
use crate::game::{Board, Evaluation};

pub struct DepthSearchAI<E> {
    max_depth: u32,
    evaluator: E,
}

impl<E> DepthSearchAI<E>
where
    E: Evaluation,
{
    pub fn new(evaluator: E, max_depth: u32) -> DepthSearchAI<E> {
        DepthSearchAI {
            max_depth,
            evaluator,
        }
    }

    fn search(&self, board: Board, depth: u32, alpha: i32, beta: i32) -> i32 {
        if depth == 0 || board.is_finished() {
            return self.evaluator.eval(&board);
        }
        let mut alpha = alpha;
        for next in board.list_next() {
            let a = -self.search(next, depth - 1, -beta, -alpha);
            if a > alpha {
                alpha = a;
            }
            if alpha >= beta {
                break;
            }
        }
        alpha
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
            let s = -self.search(next, self.max_depth, -10000, 10000);
            if s > best_score {
                best_score = s;
                best = pos_list;
            }
        }
        best
    }
}
