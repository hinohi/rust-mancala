use std::i32;

use super::base::AI;
use crate::game::{Board, Evaluation, PIT};
use std::collections::HashSet;

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

    fn search(&self, board: Board, depth: u32) -> i32 {
        if depth == 0 || board.is_finished() {
            return self.evaluator.eval(&board);
        }
        let mut best = i32::MIN;
        for next in board.list_next() {
            let s = -self.search(next, depth - 1);
            if s > best {
                best = s;
            }
        }
        best
    }
}

impl<E> AI for DepthSearchAI<E>
where
    E: Evaluation,
{
    fn deliver(&mut self, board: &Board) -> (usize, usize, u8) {
        let mut init = HashSet::new();
        init.insert((0, 0, 0));
        let s = board.get_seeds();
        for i in 0..PIT {
            for j in 0..PIT {
                if i == j {
                    continue;
                }
                for c in 1..s[0][i] {
                    init.insert((i, j, c));
                }
            }
        }
        let mut best = (0, 0, 0);
        let mut best_score = -128;
        for (pos_from, pos_to, num) in init.drain() {
            let mut copied = board.clone();
            copied.deliver(pos_from, pos_to, num);
            let s = self.search(copied, 0);
            if s > best_score {
                best_score = s;
                best = (pos_from, pos_to, num);
            }
        }
        best
    }

    fn sow(&mut self, board: &Board) -> Vec<usize> {
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
