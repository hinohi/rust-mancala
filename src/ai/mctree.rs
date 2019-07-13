use rand::Rng;

use super::{utils::random_down, AI};
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
        let per_con = (self.path_num + n - 1) / n;
        let mut best = (0, 0, std::i32::MIN);
        let mut best_pos = vec![];
        let side = board.side;
        for (board, pos) in next_lists {
            let mut win = 0;
            let mut draw = 0;
            let mut diff = 0;
            for _ in 0..per_con {
                let (s0, s1) = random_down(&mut self.random, board.clone()).last_scores();
                let score = if side == 0 {
                    i32::from(s0) - i32::from(s1)
                } else {
                    i32::from(s1) - i32::from(s0)
                };
                if score > 0 {
                    win += 1;
                } else if score == 0 {
                    draw += 1;
                }
                diff += score;
            }
            if best < (win, draw, diff) {
                best = (win, draw, diff);
                best_pos = pos;
            }
        }
        best_pos
    }
}
