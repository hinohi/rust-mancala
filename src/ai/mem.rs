use super::base::*;
use game::*;
use learn::Searcher;
use std::i32;

lazy_static! {
    static ref LEARNED: Searcher = Searcher::new("learned.msgpack");
}

pub struct MemAI {
    max_depth: u32,
    searched: u64,
    hit: u64,
}

impl MemAI {
    pub fn new(_: usize, max_depth: u32) -> MemAI {
        MemAI {
            max_depth,
            searched: 0,
            hit: 0,
        }
    }

    fn score(&self, board: &Board) -> i32 {
        let (s1, s2) = board.get_scores();
        if board.side == 0 {
            s1 as i32 - s2 as i32
        } else {
            s2 as i32 - s1 as i32
        }
    }
    fn search(&mut self, board: Board, depth: u32) -> i32 {
        self.searched += 1;
        if let Some(score) = LEARNED.get_score(&board) {
            self.hit += 1;
            return score;
        }
        if depth == 0 || board.get_state() != GameState::InBattle {
            return self.score(&board);
        }
        let mut best = i32::MIN;
        for next in board.into_iter() {
            let s = -self.search(next, depth - 1);
            if s > best {
                best = s;
            }
        }
        best
    }
}

impl AI for MemAI {
    fn think(&mut self, board: &Board) -> Vec<usize> {
        let mut best = vec![];
        let mut best_score = i32::MIN;
        let max_depth = self.max_depth;
        let (s, h) = (self.searched, self.hit);
        for (next, pos_list) in board.list_next_with_pos() {
            let s = -self.search(next, max_depth);
            if s > best_score {
                best_score = s;
                best = pos_list;
            }
        }
        println!(
            "total=({}, {}) diff=({}, {}) score={}",
            self.searched,
            self.hit,
            self.searched - s,
            self.hit - h,
            best_score,
        );
        best
    }
}
