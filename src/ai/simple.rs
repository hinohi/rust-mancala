use std::fmt::Display;
use std::io::{stderr, stdin, Write};
use std::str::FromStr;

use rand::{seq::SliceRandom, Rng};

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

pub struct RandomAI<R> {
    random: R,
}

impl<R> RandomAI<R> {
    pub fn new(random: R) -> RandomAI<R> {
        RandomAI { random }
    }
}

impl<R> AI for RandomAI<R>
where
    R: Rng,
{
    fn deliver(&mut self, board: &Board) -> (usize, usize, u8) {
        loop {
            let pos_from = self.random.gen_range(0, PIT);
            let pos_to = self.random.gen_range(0, PIT);
            let num = self.random.gen_range(0, SEED);
            if board.can_deliver(pos_from, pos_to, num) {
                return (pos_from, pos_to, num);
            }
        }
    }
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let next_list = board.list_next_with_pos().drain().collect::<Vec<_>>();
        next_list.choose(&mut self.random).unwrap().1.clone()
    }
}
