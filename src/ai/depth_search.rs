use std::i32;

use super::base::*;
use crate::game::*;

pub struct DepthSearchAI {
    max_depth: u32,
}

impl DepthSearchAI {
    pub fn new(max_depth: u32) -> DepthSearchAI {
        DepthSearchAI { max_depth }
    }

    fn score(&self, board: &Board) -> i32 {
        let (sa, sb) = board.get_scores();
        let state = board.get_state();
        if board.side == 0 {
            if state == GameState::InBattle {
                sa as i32 - sb as i32
            } else if state == GameState::WinA {
                100 + sa as i32 - sb as i32
            } else if state == GameState::WinB {
                -100 + sa as i32 - sb as i32
            } else {
                0
            }
        } else {
            if state == GameState::InBattle {
                sb as i32 - sa as i32
            } else if state == GameState::WinA {
                -100 + sb as i32 - sa as i32
            } else if state == GameState::WinB {
                100 + sb as i32 - sa as i32
            } else {
                0
            }
        }
    }
    fn search(&self, board: Board, depth: u32) -> i32 {
        if depth == 0 || board.get_state() != GameState::InBattle {
            return self.score(&board);
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

impl AI for DepthSearchAI {
    fn think(&mut self, board: &Board) -> Vec<usize> {
        let mut best = vec![];
        let mut best_score = i32::MIN;
        for (next, pos_list) in board.list_next_with_pos() {
            let s = -self.search(next, self.max_depth);
            if s > best_score {
                best_score = s;
                best = pos_list;
            }
        }
        best
    }
}
