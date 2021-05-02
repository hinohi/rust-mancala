use std::io::stdin;

use rand::{seq::SliceRandom, Rng};

use super::{utils::ab_search, Evaluator, Score, Searcher};
use crate::board::{Board, PIT};

#[derive(Debug, Clone, Default)]
pub struct Interactive<E> {
    evaluator: E,
    max_depth: usize,
}

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

impl<E> Interactive<E>
where
    E: Evaluator,
{
    pub fn new(evaluator: E, max_depth: usize) -> Interactive<E> {
        Interactive {
            evaluator,
            max_depth,
        }
    }

    fn print_suggest(&mut self, board: &Board) {
        eprintln!("suggest");
        for (pos, best) in get_suggest(board, &mut self.evaluator, self.max_depth)
            .iter()
            .enumerate()
        {
            match best {
                Some(ref best) => eprintln!("{} {:?}", pos, best),
                None => eprintln!("{} *", pos),
            }
        }
    }
}

impl<E> Searcher for Interactive<E>
where
    E: Evaluator,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        if self.max_depth > 0 {
            self.print_suggest(board);
        }
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

#[derive(Debug, Clone)]
pub struct RandomSearcher<R> {
    random: R,
}

impl<R> RandomSearcher<R> {
    pub fn new(random: R) -> RandomSearcher<R> {
        RandomSearcher { random }
    }
}

impl<R> Searcher for RandomSearcher<R>
where
    R: Rng,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let next_list = board.list_next_with_pos().drain().collect::<Vec<_>>();
        next_list.choose(&mut self.random).unwrap().1.clone()
    }
}
