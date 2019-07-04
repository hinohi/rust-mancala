use std::collections::HashMap;

use crate::game::{Board, PIT};

type Key = [u8; PIT * 2];

fn board_key(board: &Board) -> Key {
    let seeds = board.get_seeds();
    let mut key = [0; PIT * 2];
    for (i, s) in seeds[board.side].iter().enumerate() {
        key[i] = *s;
    }
    for (i, s) in seeds[1 - board.side].iter().enumerate() {
        key[PIT + i] = *s;
    }
    key
}

fn board_score(board: &Board) -> i32 {
    let (s0, s1) = board.get_scores();
    if board.side == 0 {
        s0 as i32 - s1 as i32
    } else {
        s1 as i32 - s0 as i32
    }
}
fn board_last_score(board: &Board) -> i32 {
    let (s0, s1) = board.last_scores();
    if board.side == 0 {
        s0 as i32 - s1 as i32
    } else {
        s1 as i32 - s0 as i32
    }
}

#[derive(Clone)]
pub struct Learner {
    start_depth: usize,
    db: HashMap<Key, i32>,
}

impl Learner {
    pub fn new(start_depth: usize) -> Learner {
        Learner {
            start_depth,
            db: HashMap::new(),
        }
    }

    pub fn learn(&mut self, stealing: bool) {
        let board = Board::new(stealing);
        self.search1(&board, 0);
        println!("{}", self.db.len());
    }

    fn search1(&mut self, board: &Board, depth: usize) {
        if board.is_finished() {
            return;
        }
        if depth >= self.start_depth {
            self.search2(board);
            return;
        }
        for nex in board.list_next() {
            self.search1(&nex, depth + 1);
        }
        if depth == 2 {
            println!("{}", board);
        }
    }

    fn search2(&mut self, board: &Board) -> i32 {
        let key = board_key(board);
        if let Some(score) = self.db.get(&key) {
            return *score + board_score(board);
        }
        let last = if board.is_finished() {
            board_last_score(board)
        } else {
            let mut best = std::i32::MIN;
            for nex in board.list_next() {
                let last = -self.search2(&nex);
                if best < last {
                    best = last;
                }
            }
            best
        };
        let score = board_score(board);
        self.db.insert(key, last - score);
        last
    }
}
