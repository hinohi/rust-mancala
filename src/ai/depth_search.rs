use rand::Rng;

use super::{
    utils::{ab_search, choice_with_weight, soft_max},
    Evaluator, Score, AI,
};
use crate::board::Board;

#[derive(Debug, Clone)]
pub struct DepthSearchAI<E> {
    max_depth: usize,
    evaluator: E,
}

impl<E> DepthSearchAI<E> {
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
        let mut best = Vec::new();
        let mut best_score = E::Score::MIN;
        for (next, pos_list) in next_lists {
            let s = ab_search(
                next,
                &mut self.evaluator,
                self.max_depth,
                E::Score::MIN,
                best_score.flip(),
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

#[derive(Debug, Clone)]
pub struct RandomDepthSearchAI<E, R> {
    max_depth: usize,
    weight: f64,
    evaluator: E,
    random: R,
}

impl<E, R> RandomDepthSearchAI<E, R> {
    pub fn new(
        max_depth: usize,
        weight: f64,
        evaluator: E,
        random: R,
    ) -> RandomDepthSearchAI<E, R> {
        RandomDepthSearchAI {
            max_depth,
            weight,
            evaluator,
            random,
        }
    }
}

impl<E, R> AI for RandomDepthSearchAI<E, R>
where
    E: Evaluator,
    E::Score: Into<f64>,
    R: Rng,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let mut next_lists = board.list_next_with_pos();
        if next_lists.len() == 1 {
            return next_lists.drain().next().unwrap().1;
        }
        let mut moves = Vec::with_capacity(next_lists.len());
        let mut scores = Vec::with_capacity(next_lists.len());
        for (next, pos_list) in next_lists {
            let s = ab_search(
                next,
                &mut self.evaluator,
                self.max_depth,
                E::Score::MIN,
                E::Score::MAX,
            )
            .flip();
            moves.push(pos_list);
            scores.push(s.into() * self.weight);
        }
        soft_max(&mut scores);
        moves.swap_remove(choice_with_weight(&mut self.random, &scores))
    }
}
