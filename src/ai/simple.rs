use std::fmt::Display;
use std::io::{stderr, stdin, Write};
use std::str::FromStr;

use super::base::*;
use crate::game::*;

pub struct InteractiveAI;

impl InteractiveAI {
    pub fn new() -> InteractiveAI {
        InteractiveAI {}
    }
}

fn input<T>(msg: &str) -> Option<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: Display,
{
    write!(stderr(), "{}", msg).unwrap();
    stderr().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    match buf.trim().parse() {
        Ok(v) => Some(v),
        Err(e) => {
            write!(stderr(), "{}\n", e).unwrap();
            None
        }
    }
}

impl AI for InteractiveAI {
    fn deliver(&mut self, board: &Board) -> (usize, usize, u8) {
        writeln!(stderr(), "====\n{}", board).unwrap();
        loop {
            let pos_from = if let Some(v) = input("deliver from: ") {
                v
            } else {
                continue;
            };
            let pos_to = if let Some(v) = input("deliver to: ") {
                v
            } else {
                continue;
            };
            let num = if let Some(v) = input("deliver num: ") {
                v
            } else {
                continue;
            };
            if board.can_deliver(pos_from, pos_to, num) {
                return (pos_from, pos_to, num);
            } else {
                writeln!(stderr(), "不正な移動です").unwrap();
            }
        }
    }

    fn sow(&mut self, board: &Board) -> Vec<usize> {
        writeln!(stderr(), "====\n{}", board).unwrap();
        loop {
            write!(stderr(), "your turn: ").unwrap();
            stderr().flush().unwrap();
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            match buf.trim().parse() {
                Ok(i) => match board.can_sow(i) {
                    Ok(_) => {
                        return vec![i];
                    }
                    Err(e) => write!(stderr(), "{}\n", e).unwrap(),
                },
                Err(e) => write!(stderr(), "{}\n", e).unwrap(),
            }
        }
    }
}

pub struct RandomAI;

impl RandomAI {
    pub fn new() -> RandomAI {
        RandomAI {}
    }
}

impl AI for RandomAI {
    fn deliver(&mut self, _board: &Board) -> (usize, usize, u8) {
        (0, 0, 0)
    }
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let next_map = board.list_next_with_pos();
        next_map.values().next().unwrap().clone()
    }
}
