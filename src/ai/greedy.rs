use rand::Rng;

use super::AI;
use crate::board::{Board, PIT};

pub struct GreedyAI<R> {
    random: R,
}

impl<R> GreedyAI<R>
where
    R: Rng,
{
    pub fn new(random: R) -> GreedyAI<R> {
        GreedyAI { random }
    }
}

impl<R> AI for GreedyAI<R>
where
    R: Rng,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let mut board = board.clone();
        let side = board.side();
        let mut ret = Vec::new();
        loop {
            let mut next = true;
            for (pos, &s) in board.self_seeds().iter().enumerate().rev() {
                if PIT - pos == s as usize {
                    board.sow(pos);
                    ret.push(pos);
                    if side != board.side() {
                        return ret;
                    }
                    next = false;
                    break;
                }
            }
            if next {
                break;
            }
        }
        for (pos, &s) in board.self_seeds().iter().enumerate().rev() {
            if PIT - pos < s as usize {
                ret.push(pos);
                return ret;
            }
        }
        let cond = board
            .self_seeds()
            .iter()
            .enumerate()
            .filter_map(|(pos, &s)| if s > 0 { Some(pos) } else { None })
            .collect::<Vec<_>>();
        ret.push(cond[self.random.gen_range(0, cond.len())]);
        ret
    }
}
