use std::collections::{HashMap, HashSet};
use std::fmt::{self, Write};

pub const PIT: usize = 6;
pub const SEED: u8 = 4;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Board {
    pub side: usize,
    seeds: [[u8; PIT]; 2],
    score: [u8; 2],
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
            self.seeds[1]
                .iter()
                .rev()
                .map(|p| format!("{:2}", *p))
                .collect::<Vec<String>>()
                .join("|")
        )
        .unwrap();
        if self.side == 0 {
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
    pub fn new() -> Board {
        Board {
            side: 0,
            seeds: [[SEED; PIT]; 2],
            score: [0, 0],
        }
    }

    pub fn get_scores(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }

    pub fn is_finished(&self) -> bool {
        self.seeds[0].iter().all(|s| *s == 0) || self.seeds[1].iter().all(|s| *s == 0)
    }

    pub fn get_seeds(&self) -> &[[u8; PIT]; 2] {
        &self.seeds
    }

    fn move_seed(&mut self, side: usize, pos: usize, num: usize) -> (usize, usize) {
        if pos + num <= PIT {
            for i in pos..pos + num {
                self.seeds[side][i] += 1;
            }
            return (side, pos + num - 1);
        }
        for i in pos..PIT {
            self.seeds[side][i] += 1;
        }
        if self.side == side {
            self.score[side] += 1;
            if pos + num == PIT + 1 {
                return (side, PIT);
            }
            self.move_seed(1 - side, 0, pos + num - PIT - 1)
        } else {
            self.move_seed(1 - side, 0, pos + num - PIT)
        }
    }

    pub fn can_sow(&self, pos: usize) -> Result<(), String> {
        if pos >= PIT {
            return Err(format!(
                "0から{}の間で指定してください",
                PIT - 1
            ));
        }
        if self.seeds[self.side][pos] == 0 {
            return Err("そこには石が残っていません".to_string());
        }
        Ok(())
    }

    pub fn sow(&mut self, pos: usize) {
        let num = self.seeds[self.side as usize][pos];
        self.seeds[self.side][pos] = 0;
        let (side, end_pos) = self.move_seed(self.side, pos + 1 as usize, num as usize);
        if side == self.side {
            if end_pos == PIT {
                if !self.is_finished() {
                    self.side = 1 - self.side;
                }
            } else if self.seeds[side][end_pos] == 1 {
                let opposite_pos = PIT - 1 - end_pos;
                let opposite_num = self.seeds[1 - side][opposite_pos];
                self.seeds[side][end_pos] = 0;
                self.seeds[1 - side][opposite_pos] = 0;
                self.score[side] += opposite_num + 1;
            }
        }
        self.side = 1 - self.side;
    }

    pub fn list_next(&self) -> HashSet<Board> {
        let mut set = HashSet::new();
        if self.is_finished() {
            return set;
        }
        let mut stack = vec![self.clone()];
        while let Some(board) = stack.pop() {
            for pos in 0..PIT {
                if board.seeds[board.side][pos] == 0 {
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

    pub fn list_next_with_pos(&self) -> HashMap<Board, Vec<usize>> {
        let mut map = HashMap::new();
        if self.is_finished() {
            return map;
        }
        let mut stack = vec![(self.clone(), vec![])];
        while let Some((board, pos_list)) = stack.pop() {
            for pos in 0..PIT {
                if board.seeds[board.side][pos] == 0 {
                    continue;
                }
                let mut copied = board.clone();
                let mut copied_pos = pos_list.clone();
                copied.sow(pos);
                copied_pos.push(pos);
                if !copied.is_finished() && copied.side == self.side {
                    stack.push((copied, copied_pos));
                } else {
                    map.entry(copied).or_insert(copied_pos);
                }
            }
        }
        map
    }
}

pub trait Evaluation {
    fn eval(&self, board: &Board) -> i32;
}

#[derive(Debug)]
pub struct ScoreDiffEvaluation;

impl ScoreDiffEvaluation {
    pub fn new() -> ScoreDiffEvaluation {
        ScoreDiffEvaluation {}
    }
}

impl Evaluation for ScoreDiffEvaluation {
    fn eval(&self, board: &Board) -> i32 {
        board.score[board.side] as i32 - board.score[1 - board.side] as i32
    }
}

#[derive(Debug)]
pub struct PotEvaluation;

impl PotEvaluation {
    pub fn new() -> PotEvaluation {
        PotEvaluation {}
    }
}

impl Evaluation for PotEvaluation {
    fn eval(&self, board: &Board) -> i32 {
        let p = board.seeds[board.side]
            .iter()
            .enumerate()
            .map(|(i, &s)| if i + s as usize == PIT { 1 } else { 0 })
            .sum::<i32>();
        board.score[board.side] as i32 - board.score[1 - board.side] as i32 + p
    }
}
