use rand::Rng;

use super::Searcher;
use crate::board::{Board, PIT};

#[derive(Debug, Clone)]
pub struct GreedySearcher<R> {
    stealing: bool,
    random: R,
}

impl<R> GreedySearcher<R>
where
    R: Rng,
{
    pub fn new(stealing: bool, random: R) -> GreedySearcher<R> {
        GreedySearcher { stealing, random }
    }
}

impl<R> Searcher for GreedySearcher<R>
where
    R: Rng,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let mut board = board.clone();
        let side = board.side();
        let mut ret = Vec::new();

        // 「俺のターン」
        let mut mine = true;
        while mine {
            mine = false;
            for (pos, &s) in board.self_seeds().iter().enumerate().rev() {
                if PIT - pos == s as usize {
                    board.sow(pos);
                    ret.push(pos);
                    if side != board.side() {
                        return ret;
                    }
                    mine = true;
                    break;
                }
            }
        }
        //　相手の領域にはみ出す遷移
        for (pos, &s) in board.self_seeds().iter().enumerate().rev() {
            if PIT - pos < s as usize && (self.stealing || pos < PIT - 1) {
                ret.push(pos);
                return ret;
            }
        }
        if self.stealing {
            // ランダムに
            let cond = board
                .self_seeds()
                .iter()
                .enumerate()
                .filter_map(|(pos, &s)| if s > 0 { Some(pos) } else { None })
                .collect::<Vec<_>>();
            ret.push(cond[self.random.random_range(0..cond.len())]);
        } else {
            for (pos, &s) in board.self_seeds().iter().enumerate().rev() {
                if pos < PIT - 1 && s > 0 {
                    ret.push(pos);
                    return ret;
                }
            }
            for (pos, &s) in board.self_seeds().iter().enumerate().rev() {
                if s > 0 {
                    ret.push(pos);
                    return ret;
                }
            }
        }
        ret
    }
}
