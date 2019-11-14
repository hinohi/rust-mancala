use std::collections::HashMap;
use std::fmt::{self, Write};

use fnv::FnvHashSet;

pub const PIT: usize = 6;
pub const SEED: u8 = 4;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Board {
    side: Side,
    stealing: bool,
    seeds: [[u8; PIT]; 2],
    score: [u8; 2],
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Side {
    First,
    Second,
}

use Side::*;

impl Side {
    #[inline]
    pub fn as_usize(self) -> usize {
        match self {
            First => 0,
            Second => 1,
        }
    }

    #[inline]
    pub fn turned(self) -> Side {
        match self {
            First => Second,
            Second => First,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if self.side == Second {
            s += "* |";
        } else {
            s += "  |";
        }
        write!(s, "{:2}", self.score[1]).unwrap();
        write!(
            s,
            "|{}|  |",
            self.seeds[1]
                .iter()
                .rev()
                .map(|p| format!("{:2}", *p))
                .collect::<Vec<String>>()
                .join("|")
        )
        .unwrap();
        if self.side == First {
            s += "\n* |  ";
        } else {
            s += "\n  |  ";
        }
        write!(
            s,
            "|{}|",
            self.seeds[0]
                .iter()
                .map(|p| format!("{:2}", *p))
                .collect::<Vec<String>>()
                .join("|")
        )
        .unwrap();
        write!(s, "{:2}|", self.score[0]).unwrap();
        write!(dest, "{}", s)
    }
}

impl Board {
    pub fn new(stealing: bool) -> Board {
        Board {
            side: First,
            stealing,
            seeds: [[SEED; PIT]; 2],
            score: [0, 0],
        }
    }

    pub fn from_seeds(stealing: bool, seeds: &[u8]) -> Board {
        assert_eq!(seeds.len(), PIT * 2);
        let mut s = [[SEED; PIT]; 2];
        for i in 0..PIT {
            s[0][i] = seeds[i];
        }
        for i in 0..PIT {
            s[1][i] = seeds[i + PIT];
        }
        Board {
            side: First,
            stealing,
            seeds: s,
            score: [0, 0],
        }
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn stealing(&self) -> bool {
        self.stealing
    }

    pub fn self_seeds(&self) -> &[u8] {
        &self.seeds[self.side.as_usize()]
    }

    pub fn opposite_seed(&self) -> &[u8] {
        &self.seeds[self.side.turned().as_usize()]
    }

    pub fn last_scores(&self) -> (u8, u8) {
        (
            self.score[0] + self.seeds[0].iter().sum::<u8>(),
            self.score[1] + self.seeds[1].iter().sum::<u8>(),
        )
    }

    pub fn last_score(&self) -> i8 {
        let (s0, s1) = self.last_scores();
        if self.side == Side::First {
            s0 as i8 - s1 as i8
        } else {
            s1 as i8 - s0 as i8
        }
    }

    pub fn scores(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }

    pub fn score(&self) -> i8 {
        let (s0, s1) = self.scores();
        if self.side == Side::First {
            s0 as i8 - s1 as i8
        } else {
            s1 as i8 - s0 as i8
        }
    }

    pub fn is_finished(&self) -> bool {
        self.seeds[0].iter().all(|s| *s == 0) || self.seeds[1].iter().all(|s| *s == 0)
    }

    fn move_seed(&mut self, side: Side, pos: usize, num: usize) -> (Side, usize) {
        if pos + num <= PIT {
            for i in pos..pos + num {
                self.seeds[side.as_usize()][i] += 1;
            }
            return (side, pos + num - 1);
        }
        for i in pos..PIT {
            self.seeds[side.as_usize()][i] += 1;
        }
        if self.side == side {
            self.score[side.as_usize()] += 1;
            if pos + num == PIT + 1 {
                return (side, PIT);
            }
            self.move_seed(side.turned(), 0, pos + num - PIT - 1)
        } else {
            self.move_seed(side.turned(), 0, pos + num - PIT)
        }
    }

    pub fn can_sow(&self, pos: usize) -> Result<(), String> {
        if pos >= PIT {
            return Err(format!("0から{}の間で指定してください", PIT - 1));
        }
        if self.seeds[self.side.as_usize()][pos] == 0 {
            return Err("そこには石が残っていません".to_string());
        }
        Ok(())
    }

    pub fn sow(&mut self, pos: usize) {
        let num = self.seeds[self.side.as_usize()][pos];
        self.seeds[self.side.as_usize()][pos] = 0;
        let (side, end_pos) = self.move_seed(self.side, pos + 1, num as usize);
        if side == self.side {
            if end_pos == PIT {
                if !self.is_finished() {
                    return;
                }
            } else if self.stealing && self.seeds[side.as_usize()][end_pos] == 1 {
                let opposite_pos = PIT - 1 - end_pos;
                let opposite_num = self.seeds[side.turned().as_usize()][opposite_pos];
                if opposite_num > 0 {
                    self.seeds[side.as_usize()][end_pos] = 0;
                    self.seeds[side.turned().as_usize()][opposite_pos] = 0;
                    self.score[side.as_usize()] += opposite_num + 1;
                }
            }
        }
        self.side = self.side.turned();
    }

    /// 次のターンの盤面の一覧を返す
    /// `fnv::FnvHashSet` を返すので、返り値を `iter` した順序は毎回同じであることがあることに注意
    pub fn list_next(&self) -> FnvHashSet<Board> {
        let mut set = FnvHashSet::with_capacity_and_hasher(32, Default::default());
        if self.is_finished() {
            return set;
        }
        let mut stack = Vec::with_capacity(4);
        stack.push(self.clone());
        while let Some(board) = stack.pop() {
            for (pos, &s) in board.seeds[board.side.as_usize()].iter().enumerate() {
                if s == 0 {
                    continue;
                }
                let mut copied = board.clone();
                copied.sow(pos);
                if copied.side == self.side {
                    stack.push(copied);
                } else {
                    set.insert(copied);
                }
            }
        }
        set
    }

    /// 次のターンの盤面とその盤面にするために必要な打ち手のペアの一覧を返す
    /// `std::collections::HashMap` を返すので、返り値を `iter` した順序は毎回異なることを期待して良い
    pub fn list_next_with_pos(&self) -> HashMap<Board, Vec<usize>> {
        let mut map = HashMap::with_capacity(32);
        if self.is_finished() {
            return map;
        }
        let mut stack = Vec::with_capacity(4);
        stack.push((self.clone(), Vec::with_capacity(1)));
        while let Some((board, pos_list)) = stack.pop() {
            for (pos, &s) in board.seeds[board.side.as_usize()].iter().enumerate() {
                if s == 0 {
                    continue;
                }
                let mut copied = board.clone();
                let mut copied_pos = pos_list.clone();
                copied.sow(pos);
                copied_pos.push(pos);
                if copied.side == self.side {
                    stack.push((copied, copied_pos));
                } else {
                    map.entry(copied).or_insert(copied_pos);
                }
            }
        }
        map
    }
}

pub fn compact_key(board: &Board) -> u64 {
    let s = board.self_seeds();
    let k0 = (u64::from(s[0]) << 10) + (u64::from(s[1]) << 5) + u64::from(s[2]);
    let k1 = (u64::from(s[3]) << 10) + (u64::from(s[4]) << 5) + u64::from(s[5]);
    let s = board.opposite_seed();
    let k2 = (u64::from(s[0]) << 10) + (u64::from(s[1]) << 5) + u64::from(s[2]);
    let k3 = (u64::from(s[3]) << 10) + (u64::from(s[4]) << 5) + u64::from(s[5]);
    (k0 << 48) + (k1 << 32) + (k2 << 16) + k3
}

pub fn from_compact_key(key: u64) -> [u8; 12] {
    let mut ret = [0; 12];

    ret[11] = (key & 0b11111) as u8;
    let key = key >> 5;
    ret[10] = (key & 0b11111) as u8;
    let key = key >> 5;
    ret[9] = (key & 0b11111) as u8;
    let key = key >> 6;

    ret[8] = (key & 0b11111) as u8;
    let key = key >> 5;
    ret[7] = (key & 0b11111) as u8;
    let key = key >> 5;
    ret[6] = (key & 0b11111) as u8;
    let key = key >> 6;

    ret[5] = (key & 0b11111) as u8;
    let key = key >> 5;
    ret[4] = (key & 0b11111) as u8;
    let key = key >> 5;
    ret[3] = (key & 0b11111) as u8;
    let key = key >> 6;

    ret[2] = (key & 0b11111) as u8;
    let key = key >> 5;
    ret[1] = (key & 0b11111) as u8;
    let key = key >> 5;
    ret[0] = (key & 0b11111) as u8;
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sow_with_stealing() {
        // skip
        if !(PIT == 6 && SEED == 4) {
            return;
        }
        let mut board = Board::new(true);
        board.sow(2);
        assert_eq!(board.side, First);
        assert_eq!(board.scores(), (1, 0));
        board.sow(5);
        assert_eq!(board.side, Second);
        assert_eq!(board.scores(), (2, 0));
        board.sow(1);
        assert_eq!(board.side, Second);
        assert_eq!(board.scores(), (2, 1));
        board.sow(5);
        assert_eq!(board.side, First);
        assert_eq!(board.scores(), (2, 2));
        board.sow(1);
        assert_eq!(board.side, First);
        assert_eq!(board.scores(), (3, 2));
        board.sow(5);
        assert_eq!(board.side, First);
        assert_eq!(board.scores(), (4, 2));
        board.sow(0);
        assert_eq!(board.side, Second);
        assert_eq!(board.scores(), (10, 2));
        assert_eq!(board.last_scores(), (29, 19));
    }

    #[test]
    fn sow_no_stealing() {
        // skip
        if !(PIT == 6 && SEED == 4) {
            return;
        }
        let mut board = Board::new(false);
        board.sow(2);
        assert_eq!(board.side, First);
        assert_eq!(board.scores(), (1, 0));
        board.sow(5);
        assert_eq!(board.side, Second);
        assert_eq!(board.scores(), (2, 0));
        board.sow(1);
        assert_eq!(board.side, Second);
        assert_eq!(board.scores(), (2, 1));
        board.sow(5);
        assert_eq!(board.side, First);
        assert_eq!(board.scores(), (2, 2));
        board.sow(1);
        assert_eq!(board.side, First);
        assert_eq!(board.scores(), (3, 2));
        board.sow(5);
        assert_eq!(board.side, First);
        assert_eq!(board.scores(), (4, 2));
        board.sow(0);
        assert_eq!(board.side, Second);
        assert_eq!(board.scores(), (4, 2));
        assert_eq!(board.last_scores(), (24, 24));
    }

    #[test]
    fn test_compact_key() {
        // skip
        if !(PIT == 6 && SEED == 4) {
            return;
        }

        fn test(b: &Board) {
            let k = compact_key(b);
            let v = from_compact_key(k);
            println!("{}\n{} {:?}", b, k, v);
            for i in 0..6 {
                assert_eq!(b.self_seeds()[i], v[i]);
            }
            for i in 0..6 {
                assert_eq!(b.opposite_seed()[i], v[6 + i]);
            }
        }

        let mut board = Board::new(false);
        test(&board);
        board.sow(2);
        test(&board);
        board.sow(5);
        test(&board);
        board.sow(1);
        test(&board);
        board.sow(3);
        test(&board);
        board.sow(4);
        test(&board);
    }

    #[test]
    fn test_score() {
        // skip
        if !(PIT == 6 && SEED == 4) {
            return;
        }
        let mut board = Board::new(true);
        assert_eq!(board.score(), 0);
        assert_eq!(board.last_score(), 0);
        board.sow(2);
        assert_eq!(board.score(), 1);
        assert_eq!(board.last_score(), 0);
        board.sow(5);
        assert_eq!(board.score(), -2);
        assert_eq!(board.last_score(), (5 - 1) * 2);
    }

    #[test]
    fn from_seeds() {
        let mut board = Board::new(true);
        board.sow(2);
        board.sow(5);

        let key = compact_key(&board);
        let seeds = from_compact_key(key);
        let b = Board::from_seeds(true, &seeds);

        assert_eq!(board.self_seeds(), b.self_seeds());
        assert_eq!(board.opposite_seed(), b.opposite_seed());
    }
}
