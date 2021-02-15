use fnv::FnvHashMap;
use rand::Rng;

use crate::ai::{RandomAI, AI};
use crate::board::{compact_key, Board};

fn raw_scores(board: &Board) -> i8 {
    board.score()
}

fn seed_scores(board: &Board) -> i8 {
    let l0 = board.self_seeds().iter().sum::<u8>() as i8;
    let l1 = board.opposite_seed().iter().sum::<u8>() as i8;
    l0 - l1
}

const LIMIT: usize = 3_758_096_384;

pub fn search(data: &mut FnvHashMap<u64, (i8, u8)>, board: Board, depth: u8) -> Option<(i8, u8)> {
    if data.len() == LIMIT {
        return None;
    }
    let key = compact_key(&board);
    if let Some((l, d)) = data.get(&key) {
        return Some((raw_scores(&board) + *l, *d));
    }
    if board.is_finished() {
        let s = raw_scores(&board);
        let l = seed_scores(&board);
        data.insert(key, (l, 0));
        return Some((s + l, 0));
    }
    if depth == 0 {
        return None;
    }
    let mut best_score = -128;
    let mut best_depth = 127;
    for next in board.list_next() {
        let a = search(data, next, depth - 1);
        match a {
            None => return None,
            Some((s, d)) => {
                let s = -s;
                let d = d + 1;
                if s > best_score {
                    best_score = s;
                    best_depth = d;
                } else if s == best_score && best_depth > d {
                    best_depth = d
                }
            }
        }
    }
    if data.len() == LIMIT {
        return None;
    }
    data.insert(key, (best_score - raw_scores(&board), best_depth));
    Some((best_score, best_depth))
}

pub fn to_finish<R: Rng>(stealing: bool, ai: &mut RandomAI<R>) -> Vec<Board> {
    let mut board = Board::new(stealing);
    let mut ret = vec![board.clone()];
    while !board.is_finished() {
        let pos_list = ai.sow(&board);
        for pos in pos_list {
            board.sow(pos);
        }
        ret.push(board.clone());
    }
    ret
}
