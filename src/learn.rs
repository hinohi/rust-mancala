use std::collections::BTreeMap;
use game::*;


pub struct Searcher {
    map: BTreeMap<[u8; PIT * 2], i32>,
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {
            map: BTreeMap::new(),
        }
    }

    fn _get_score(&self, board: &Board) -> i32 {
        let (s1, s2) = board.get_scores();
        if board.side == 0 {
            s1 as i32  - s2 as i32
        } else {
            s2 as i32 - s1 as i32
        }
    }

    fn search(&mut self, board: &Board, depth: u32) -> Option<i32> {
        let key = board.get_rest_stone();
        if self.map.contains_key(&key) {
            return Some(self._get_score(board) + self.map[&key]);
        }
        if board.get_state() != GameState::InBattle {
            self.map.insert(key, 0);
            return Some(self._get_score(board));
        }
        if depth == 0 {
            return None;
        }
        let mut best = std::i32::MIN;
        let mut ok = true;
        for next in board.into_iter() {
            let score = self.search(&next, depth - 1);
            if !ok || score.is_none() {
                ok = false;
                continue;
            }
            let score = -score.unwrap();
            if best < score {
                best = score;
            }
        }
        if ok {
            let n_score = self._get_score(board);
            self.map.insert(key, best - n_score);
            Some(best)
        } else {
            None
        }
    }

    pub fn single_run(&mut self) {
        let mut board = Board::new();
        while board.get_state() == GameState::InBattle {
            self.search(&board, 2);
            let next_map = board.list_next_with_pos();
            for pos in next_map.values().next().unwrap() {
                board.move_one(*pos);
            }
        }
        println!("{}", self.map.len());
    }

    pub fn dump(&self) {
        for (key, value) in &self.map {
            println!("{:?} {}", key, value);
        }
    }
}