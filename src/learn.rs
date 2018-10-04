use game::*;
use rmp_serde;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::i32;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Searcher {
    map: BTreeMap<[u8; PIT * 2], i32>,
}

impl Searcher {
    pub fn new(file: &str) -> Searcher {
        let f = File::open(file).unwrap();
        let reader = BufReader::new(f);
        rmp_serde::from_read(reader).unwrap()
    }

    pub fn get_score(&self, board: &Board) -> Option<i32> {
        let key = board.get_rest_stone();
        if let Some(s) = self.map.get(&key) {
            Some(s + self._get_score(board))
        } else {
            None
        }
    }

    fn _get_score(&self, board: &Board) -> i32 {
        let (s1, s2) = board.get_scores();
        if board.side == 0 {
            s1 as i32 - s2 as i32
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
        let mut best = i32::MIN;
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
            if self.search(&board, 4).is_some() {
                return;
            }
            let next_map = board.list_next_with_pos();
            for pos in next_map.values().next().unwrap() {
                board.move_one(*pos);
            }
        }
    }

    pub fn info(&self) {
        println!("{}", self.map.len());
    }

    pub fn dump(&self, file: &str) {
        let f = File::create(file).unwrap();
        let writer = BufWriter::new(f);
        let mut s = rmp_serde::Serializer::new(writer);
        self.serialize(&mut s).unwrap();
    }
}
