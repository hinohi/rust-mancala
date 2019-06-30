use std::io::{stderr, stdin, Write};

use rand::{seq::SliceRandom, Rng};

use super::base::AI;
use super::utils::ab_search;
use crate::game::{Board, Evaluation, ScoreDiffEvaluation, PIT};

pub struct InteractiveAI;

fn get_suggest<E: Evaluation>(board: &Board, eval: &E, max_depth: usize) -> Vec<Option<i32>> {
    let mut ret = vec![None; PIT];
    for (next, pos_list) in board.list_next_with_pos() {
        let s = -ab_search(next, eval, max_depth, -10000, 10000);
        let pos = pos_list[0];
        match ret[pos] {
            None => ret[pos] = Some(s),
            Some(best) if best < s => ret[pos] = Some(s),
            _ => (),
        }
    }
    ret
}

impl InteractiveAI {
    pub fn new() -> InteractiveAI {
        InteractiveAI {}
    }

    fn print_suggest(&self, board: &Board) {
        let eval = ScoreDiffEvaluation::new();
        println!("suggest");
        for max_depth in 1..9 {
            print!("{} |", max_depth);
            for best in get_suggest(board, &eval, max_depth) {
                match best {
                    Some(best) => print!("{:4}", best),
                    None => print!("    *"),
                }
            }
            println!("|");
        }
    }
}

impl AI for InteractiveAI {
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        writeln!(stderr(), "====\n{}", board).unwrap();
        self.print_suggest(board);
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
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let next_list = board.list_next_with_pos().drain().collect::<Vec<_>>();
        next_list.choose(&mut self.random).unwrap().1.clone()
    }
}
