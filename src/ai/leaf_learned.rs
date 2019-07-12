use std::io::{self, Read};

use fnv::FnvHashMap;
use rand::Rng;

use super::base::AI;
use crate::game::{Board, PIT, SEED};

type Key = [u8; PIT * 2];

fn board_key(board: &Board) -> Key {
    let seeds = board.seeds();
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
    let (s0, s1) = board.scores();
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
    db: FnvHashMap<Key, i8>,
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
            db: FnvHashMap::default(),
        }
    }

    pub fn dump<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        let n = self.db.len() as u64;
        writer.write_all(&n.to_le_bytes())?;
        for (key, value) in self.db.iter() {
            writer.write_all(key)?;
            writer.write_all(&[*value as u8])?;
        }
        Ok(())
    }

    pub fn restore<Re: io::Read>(&mut self, reader: &mut Re) -> io::Result<()> {
        let n = {
            let mut buf = [0; 8];
            reader.read_exact(&mut buf)?;
            u64::from_le_bytes(buf) as usize
        };
        self.db = FnvHashMap::with_capacity_and_hasher(n, Default::default());
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

lazy_static! {
    static ref DB: FnvHashMap<Key, i8> = read_db(true);
}

fn read_db(stealing: bool) -> FnvHashMap<Key, i8> {
    let name = format!("p{}s{}_{}.db", PIT, SEED, stealing);
    let mut f = match std::fs::File::open(name) {
        Ok(f) => io::BufReader::new(f),
        Err(_) => return FnvHashMap::default(),
    };
    let n = {
        let mut n: u64 = 0;
        let mut buf = [0; 8];
        f.read_exact(&mut buf).unwrap();
        for i in 0..8 {
            n += (buf[i] as u64) << (i as u64 * 8);
        }
        n as usize / 4
    };
    let mut ret = FnvHashMap::with_capacity_and_hasher(n, Default::default());
    let mut buf = [0; PIT * 2 + 1];
    for _ in 0..n {
        match f.read_exact(&mut buf) {
            Err(_) => return ret,
            Ok(_) => (),
        }
        let mut key = [0; PIT * 2];
        for (pos, &s) in buf[..PIT].iter().enumerate() {
            key[pos] = s;
        }
        for (pos, &s) in buf[PIT..PIT * 2].iter().enumerate() {
            key[PIT + pos] = s;
        }
        ret.insert(key, buf[PIT * 2] as i8);
    }
    ret
}

fn db_key(board: &Board) -> Key {
    board_key(board)
}

pub struct LearnedMCTree<R: Rng> {
    path_num: usize,
    random: R,
    hit: u64,
    show_hit: bool,
}

impl<R> LearnedMCTree<R>
where
    R: Rng,
{
    pub fn new(path_num: usize, random: R) -> LearnedMCTree<R> {
        LearnedMCTree {
            path_num,
            random,
            hit: 0,
            show_hit: false,
        }
    }

    pub fn show_hit(&mut self, show: bool) {
        self.show_hit = show;
    }

    fn random_down(&mut self, board: Board) -> (i8, usize) {
        let mut board = board;
        loop {
            let key = db_key(&board);
            if let Some(score) = DB.get(&key) {
                self.hit += 1;
                return (*score + board_score(&board), board.side);
            }
            let mut next_list = board.list_next().drain().collect::<Vec<_>>();
            if next_list.is_empty() {
                break;
            }
            let idx = self.random.gen_range(0, next_list.len());
            board = next_list.swap_remove(idx);
        }
        (board_last_score(&board), board.side)
    }
}

impl<R> AI for LearnedMCTree<R>
where
    R: Rng,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        self.hit = 0;
        let mut next_lists = board.list_next_with_pos();
        let n = next_lists.len();
        if n == 1 {
            return next_lists.drain().next().unwrap().1;
        }
        let per_con = (self.path_num + n - 1) / n;
        let mut best = (0, 0, std::i8::MIN);
        let mut best_pos = vec![];
        for (board, pos) in next_lists {
            let mut win = 0;
            let mut draw = 0;
            let mut diff = 0;
            for _ in 0..per_con {
                let (s, side) = self.random_down(board.clone());
                let score = if side == board.side { -s } else { s };
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
        if self.show_hit {
            println!("{} / {}", self.hit, per_con * n);
        }
        best_pos
    }
}
