use std::collections::HashMap;
use std::io;

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

fn board_score(board: &Board) -> i8 {
    let (s0, s1) = board.get_scores();
    if board.side == 0 {
        s0 as i8 - s1 as i8
    } else {
        s1 as i8 - s0 as i8
    }
}
fn board_last_score(board: &Board) -> i8 {
    let (s0, s1) = board.last_scores();
    if board.side == 0 {
        s0 as i8 - s1 as i8
    } else {
        s1 as i8 - s0 as i8
    }
}

#[derive(Clone)]
pub struct Learner<R> {
    random: R,
    stealing: bool,
    back_depth: isize,
    db: HashMap<Key, i8>,
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

    pub fn dump<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        let n = self.db.len() as u64;
        {
            let mut n = n;
            let mut buf = [0; 8];
            for i in 0..8 {
                buf[i] = (n & 0xFF) as u8;
                n >>= 8;
            }
            writer.write_all(&buf)?;
        };
        for (key, value) in self.db.iter() {
            writer.write_all(key)?;
            writer.write_all(&[*value as u8])?;
        }
        Ok(())
    }

    pub fn restore<Re: io::Read>(&mut self, reader: &mut Re) -> io::Result<()> {
        let n = {
            let mut n: u64 = 0;
            let mut buf = [0; 8];
            reader.read_exact(&mut buf)?;
            for i in 0..8 {
                n += (buf[i] as u64) << (i as u64 * 8);
            }
            n as usize
        };
        self.db = HashMap::with_capacity(n);
        for _ in 0..n {
            let mut key = [0; PIT * 2];
            reader.read_exact(&mut key)?;
            let mut score = [0; 1];
            reader.read_exact(&mut score)?;
            self.db.insert(key, score[0] as i8);
        }
        Ok(())
    }

    pub fn learn(&mut self, num: usize) {
        let board = Board::new(self.stealing);
        for i in 0..num {
            self.search1(&board, 0);
            if i % (num / 100) == 0 {
                println!("{} / {}", i, num);
            }
        }
        println!("{}", self.db.len());
    }

    fn search1(&mut self, board: &Board, depth: isize) -> isize {
        let key = board_key(board);
        if self.db.contains_key(&key) {
            return depth - 1;
        }
        let mut next_list = board.list_next().drain().collect::<Vec<_>>();
        if next_list.is_empty() {
            return depth - self.back_depth;
        }
        let idx = self.random.gen_range(0, next_list.len());
        let nex = next_list.swap_remove(idx);
        let back = self.search1(&nex, depth + 1);
        if back == depth {
            self.search2(board, 0);
        }
        back
    }

    fn search2(&mut self, board: &Board, depth: usize) -> Option<i8> {
        if depth == 20 {
            return None;
        }
        let key = board_key(board);
        if let Some(score) = self.db.get(&key) {
            return Some(*score + board_score(board));
        }
        let last = if board.is_finished() {
            board_last_score(board)
        } else {
            let mut best = std::i8::MIN;
            for nex in board.list_next() {
                let last = if let Some(s) = self.search2(&nex, depth + 1) {
                    -s
                } else {
                    return None;
                };
                if best < last {
                    best = last;
                }
            }
            best
        };
        let score = board_score(board);
        self.db.insert(key, last - score);
        Some(last)
    }
}
