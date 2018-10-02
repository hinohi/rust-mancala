use std::collections::{HashMap, HashSet};
use std::fmt::{self, Write};

pub const PIT: usize = 6;
pub const STONE: u8 = 4;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Board {
    pub side: u8,
    pits: [[u8; PIT]; 2],
    score: [u8; 2],
}

#[derive(Debug, Eq, PartialEq)]
pub enum GameState {
    InBattle,
    WinA,
    WinB,
    Draw,
}

impl fmt::Display for Board {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if self.side == 1 {
            s += "* |";
        } else {
            s += "  |";
        }
        write!(s, "{:2}", self.score[1]).unwrap();
        write!(
            s,
            "|{}|  |",
            self.pits[1]
                .iter()
                .rev()
                .map(|p| format!("{:2}", *p))
                .collect::<Vec<String>>()
                .join("|")
        ).unwrap();
        if self.side == 0 {
            s += "\n* |  ";
        } else {
            s += "\n  |  ";
        }
        write!(
            s,
            "|{}|",
            self.pits[0]
                .iter()
                .map(|p| format!("{:2}", *p))
                .collect::<Vec<String>>()
                .join("|")
        ).unwrap();
        write!(s, "{:2}|", self.score[0]).unwrap();
        write!(dest, "{}", s)
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            side: 0,
            pits: [[STONE; PIT]; 2],
            score: [0, 0],
        }
    }

    pub fn get_state(&self) -> GameState {
        if self.pits[0].iter().sum::<u8>() == 0 || self.pits[1].iter().sum::<u8>() == 0 {
            if self.score[0] > self.score[1] {
                return GameState::WinA;
            } else if self.score[0] < self.score[1] {
                return GameState::WinB;
            } else {
                return GameState::Draw;
            }
        }
        GameState::InBattle
    }

    pub fn get_scores(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }

    fn _move_stone(&mut self, side: usize, pos: usize, num: usize) -> (usize, usize) {
        if pos + num <= PIT {
            for i in pos..pos + num {
                self.pits[side][i] += 1;
            }
            return (side, pos + num - 1);
        }
        for i in pos..PIT {
            self.pits[side][i] += 1;
        }
        if self.side == side as u8 {
            self.score[side] += 1;
            if pos + num == PIT + 1 {
                return (side, PIT);
            }
            self._move_stone(1 - side, 0, pos + num - PIT - 1)
        } else {
            self._move_stone(1 - side, 0, pos + num - PIT)
        }
    }

    pub fn check_pos(&self, pos: usize) -> Result<(), String> {
        if pos >= PIT {
            return Err(format!(
                "0から{}の間で指定してください",
                PIT - 1
            ));
        }
        if self.pits[self.side as usize][pos] == 0 {
            return Err("そこには石が残っていません".to_string());
        }
        Ok(())
    }

    pub fn move_one(&mut self, pos: usize) {
        debug_assert!(pos < PIT);
        debug_assert!(self.pits[self.side as usize][pos] > 0);
        debug_assert!(self.get_state() == GameState::InBattle);
        let num = self.pits[self.side as usize][pos];
        self.pits[self.side as usize][pos] = 0;
        let side = self.side as usize;
        let (side, end_pos) = self._move_stone(side, pos + 1 as usize, num as usize);
        let next_side;
        if side as u8 == self.side {
            if end_pos == PIT {
                next_side = self.side;
            } else {
                next_side = 1 - self.side;
                if self.pits[side][end_pos] == 1 {
                    let opposite_pos = PIT - 1 - end_pos;
                    let opposite_num = self.pits[1 - side][opposite_pos];
                    self.pits[1 - side][opposite_pos] = 0;
                    self.score[side] += opposite_num;
                }
            }
        } else {
            next_side = 1 - self.side;
        }
        self.side = next_side;
    }

    pub fn list_next(&self) -> HashSet<Board> {
        let mut set = HashSet::new();
        if self.get_state() != GameState::InBattle {
            return set;
        }
        let mut stack = vec![self.clone()];
        while !stack.is_empty() {
            let board = stack.pop().unwrap();
            for pos in 0..PIT {
                if !board.check_pos(pos).is_ok() {
                    continue;
                }
                let mut copied = board.clone();
                copied.move_one(pos);
                if copied.side == self.side {
                    stack.push(copied);
                } else {
                    set.insert(copied);
                }
            }
        }
        set
    }

    pub fn list_next_with_pos(&self) -> HashMap<Board, Vec<usize>> {
        let mut map = HashMap::new();
        if self.get_state() != GameState::InBattle {
            return map;
        }
        let mut stack = vec![(self.clone(), vec![])];
        while !stack.is_empty() {
            let (board, pos_list) = stack.pop().unwrap();
            for pos in 0..PIT {
                if !board.check_pos(pos).is_ok() {
                    continue;
                }
                let mut copied = board.clone();
                let mut copied_pos = pos_list.clone();
                copied.move_one(pos);
                copied_pos.push(pos);
                if copied.get_state() == GameState::InBattle && copied.side == self.side {
                    stack.push((copied, copied_pos));
                } else {
                    map.entry(copied).or_insert(copied_pos);
                }
            }
        }
        map
    }
}

pub struct NextBoardIter {
    stack: Vec<(Board, usize)>,
}

impl<'a> IntoIterator for &'a Board {
    type Item = Board;
    type IntoIter = NextBoardIter;
    fn into_iter(self) -> Self::IntoIter {
        NextBoardIter {
            stack: vec![(self.clone(), 0)],
        }
    }
}

impl Iterator for NextBoardIter {
    type Item = Board;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let (board, start) = self.stack.pop().unwrap();
            if board.get_state() != GameState::InBattle {
                continue;
            }
            for pos in start..PIT {
                if !board.check_pos(pos).is_ok() {
                    continue;
                }
                let mut copied = board.clone();
                copied.move_one(pos);
                if copied.get_state() == GameState::InBattle && copied.side == board.side {
                    self.stack.push((copied, 0));
                } else {
                    if pos != PIT - 1 {
                        self.stack.push((board, pos + 1))
                    }
                    return Some(copied);
                }
            }
        }
        None
    }
}
