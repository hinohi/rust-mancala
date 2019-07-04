use std::collections::HashMap;

use rand::Rng;

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
pub struct Learner<R> {
    random: R,
    stealing: bool,
    back_depth: isize,
    db: HashMap<Key, i32>,
}

impl<R> Learner<R>
where
    R: Rng,
{
    pub fn new(random: R, stealing: bool, back_depth: isize) -> Learner<R> {
        Learner {
            random,
            stealing,
            back_depth,
            db: HashMap::new(),
        }
    }

    pub fn learn(&mut self, num: usize) {
        let board = Board::new(self.stealing);
        for _ in 0..num {
            self.search1(&board, 0);
        }
        println!("{}", self.db.len());
    }

    fn search1(&mut self, board: &Board, depth: isize) -> isize {
        let mut next_list = board.list_next().drain().collect::<Vec<_>>();
        if next_list.is_empty() {
            return depth - self.back_depth;
        }
        let idx = self.random.gen_range(0, next_list.len());
        let nex = next_list.swap_remove(idx);
        let back = self.search1(&nex, depth + 1);
        if back == depth {
            let s = self.search2(board);
            println!("{}\n{}", board, s);
        }
        back
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
