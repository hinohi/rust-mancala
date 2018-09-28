use std::fmt::{self, Write};
use std::string::String;

const PIT: usize = 6;
const STONE: usize = 4;

#[derive(Debug)]
struct Board {
    side: usize,
    pits: [[usize; PIT]; 2],
    score: [usize; 2],
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
        write!(s, "|{}|  |", self.pits[1].iter()
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
        write!(s, "|{}|", self.pits[0].iter()
            .map(|p| format!("{:2}", *p))
            .collect::<Vec<String>>()
            .join("|")
        ).unwrap();
        write!(s, "{:2}|\n", self.score[0]).unwrap();
        write!(dest, "{}", s)
    }
}

impl Board {
    fn new() -> Board {
        Board {
            side: 0,
            pits: [[STONE; PIT]; 2],
            score: [0; 2],
        }
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
        if self.side == side {
            self.score[side] += 1;
            if pos + num == PIT + 1 {
                return (side, PIT);
            }
            self._move_stone(1 - side, 0, pos + num - PIT - 1)
        } else {
            self._move_stone(1 - side, 0, pos + num - PIT)
        }
    }

    fn move_one(&mut self, pos: usize) -> usize {
        debug_assert!(pos < PIT);
        debug_assert!(self.pits[self.side][pos] > 0);
        let num = self.pits[self.side][pos];
        self.pits[self.side][pos] = 0;
        let side = self.side;
        let (side, end_pos) = self._move_stone(side, pos + 1 as usize, num as usize);
        let next_side;
        if side == self.side {
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
        next_side
    }
}

fn main() {
    let mut board = Board::new();
    println!("{}", board);
    board.move_one(2);
    println!("{}", board);
    board.move_one(5);
    println!("{}", board);
    board.move_one(5);
    println!("{}", board);
    board.move_one(0);
    println!("{}", board);
}
