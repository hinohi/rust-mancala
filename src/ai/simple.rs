use std::io::{stderr, stdin, Write};

use super::base::*;
use game::*;

pub struct InteractiveAI {}

impl InteractiveAI {
    pub fn new() -> InteractiveAI {
        InteractiveAI {}
    }
}

impl AI for InteractiveAI {
    fn think(&mut self, board: &Board) -> Vec<usize> {
        writeln!(stderr(), "====\n{}", board).unwrap();
        loop {
            write!(stderr(), "your turn: ").unwrap();
            stderr().flush().unwrap();
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            match buf.trim().parse() {
                Ok(i) => match board.check_pos(i) {
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

pub struct RandomAI {}

impl RandomAI {
    pub fn new() -> RandomAI {
        RandomAI {}
    }
}

impl AI for RandomAI {
    fn think(&mut self, board: &Board) -> Vec<usize> {
        let mut next_map = board.list_next_with_pos();
        if next_map.is_empty() {
            println!("{}", board);
        }
        let key = next_map.keys().next().unwrap().clone();
        next_map.remove(&key).unwrap()
    }
}
