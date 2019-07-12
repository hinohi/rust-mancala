use rand::Rng;

use super::AI;
use crate::board::Board;

#[inline]
fn flip_score(board: &Board) -> i32 {
    if board.side == 0 {
        1
    } else {
        -1
    }
}

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

    fn random_down(&mut self, board: Board) -> i32 {
        let mut board = board;
        loop {
            let mut next_list = board.list_next().drain().collect::<Vec<_>>();
            if next_list.is_empty() {
                break;
            }
            let idx = self.random.gen_range(0, next_list.len());
            board = next_list.swap_remove(idx);
        }
        let (s0, s1) = board.last_scores();
        s0 as i32 - s1 as i32
    }
}

impl<R> AI for MCTree<R>
where
    R: Rng,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let mut next_lists = board.list_next_with_pos();
        if next_lists.len() == 1 {
            return next_lists.drain().next().unwrap().1;
        }
        let per_con = (self.path_num + next_lists.len() - 1) / next_lists.len();
        let mut best = (0, 0, std::i32::MIN);
        let mut best_pos = vec![];
        let flip = flip_score(board);
        for (board, pos) in next_lists {
            let mut win = 0;
            let mut draw = 0;
            let mut diff = 0;
            for _ in 0..per_con {
                let score = self.random_down(board.clone()) * flip;
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
