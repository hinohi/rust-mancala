use std::io::stdin;

use rand::{seq::SliceRandom, Rng};

use super::{evaluator::ScoreDiffEvaluator, utils::ab_search, Evaluator, Score, AI};
use crate::board::{Board, PIT};

#[derive(Debug, Default)]
pub struct InteractiveAI;

fn get_suggest<E: Evaluator>(
    board: &Board,
    eval: &mut E,
    max_depth: usize,
) -> Vec<Option<E::Score>> {
    let mut ret = vec![None; PIT];
    for (next, pos_list) in board.list_next_with_pos() {
        let s = ab_search(next, eval, max_depth, E::Score::MIN, E::Score::MAX).flip();
        let pos = pos_list[0];
        match ret.get(pos) {
            Some(None) => ret[pos] = Some(s),
            Some(Some(best)) if best < &s => ret[pos] = Some(s),
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
        let mut eval = ScoreDiffEvaluator::new();
        eprintln!("suggest");
        for max_depth in 1..9 {
            eprint!("{} |", max_depth);
            for best in get_suggest(board, &mut eval, max_depth) {
                match best {
                    Some(ref best) => eprint!("{:4}", best),
                    None => eprint!("    *"),
                }
            }
            eprintln!("|");
        }
    }
}

impl AI for InteractiveAI {
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        self.print_suggest(board);
        loop {
            eprint!("your turn: ");
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            match buf.trim().parse() {
                Ok(i) => match board.can_sow(i) {
                    Ok(_) => {
                        return vec![i];
                    }
                    Err(e) => eprintln!("{}", e),
                },
                Err(e) => eprintln!("{}", e),
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
