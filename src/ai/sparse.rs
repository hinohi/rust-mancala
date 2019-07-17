use fnv::{FnvHashMap, FnvHashSet};
use rand::Rng;

use super::AI;
use crate::board::{Board, Side};

pub struct SparseDepthSearchAI<R> {
    first_depth: usize,
    num: usize,
    random: R,
    network: FnvHashMap<Board, Node>,
}

enum Node {
    Next(FnvHashSet<Board>),
    Score(i32),
}

impl<R> SparseDepthSearchAI<R>
where
    R: Rng,
{
    pub fn new(first_depth: usize, num: usize, random: R) -> SparseDepthSearchAI<R> {
        SparseDepthSearchAI {
            first_depth,
            num,
            random,
            network: FnvHashMap::default(),
        }
    }

    fn grow1(&mut self, board: &Board, depth: usize) {
        if depth == 0 {
            for _ in 0..self.num {
                self.grow2(board);
            }
            return;
        }
        for nex in board.list_next() {
            self.network
                .entry(nex.clone())
                .or_insert_with(|| Node::Next(FnvHashSet::default()));
            self.grow1(&nex, depth - 1);
            match self.network.get_mut(board) {
                Some(Node::Next(s)) => {
                    s.insert(nex);
                }
                _ => unreachable!(),
            }
        }
    }

    fn grow2(&mut self, board: &Board) {
        let mut board = board.clone();
        loop {
            let mut next_list = board.list_next().drain().collect::<Vec<_>>();
            if next_list.is_empty() {
                break;
            }
            let idx = self.random.gen_range(0, next_list.len());
            let next = next_list.swap_remove(idx);
            self.network
                .entry(next.clone())
                .or_insert_with(|| Node::Next(FnvHashSet::default()));
            match self.network.get_mut(&board) {
                Some(Node::Next(s)) => {
                    s.insert(next.clone());
                }
                _ => unreachable!(),
            }
            board = next;
        }
    }

    fn search(&mut self, board: &Board) -> i32 {
        let next_list = match self.network.get(board) {
            Some(Node::Score(s)) => return *s,
            Some(Node::Next(s)) => s.clone(),
            _ => unreachable!(),
        };
        if next_list.is_empty() {
            let (s0, s1) = board.last_scores();
            return if board.side() == Side::First {
                i32::from(s0) - i32::from(s1)
            } else {
                i32::from(s1) - i32::from(s0)
            };
        }
        let mut best = std::i32::MIN;
        for nex in next_list {
            let s = -self.search(&nex);
            if best < s {
                best = s;
            }
        }
        *self.network.get_mut(board).unwrap() = Node::Score(best);
        best
    }
}

impl<R> AI for SparseDepthSearchAI<R>
where
    R: Rng,
{
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        self.network.clear();
        self.network
            .insert(board.clone(), Node::Next(FnvHashSet::default()));
        self.grow1(&board, self.first_depth);
        let mut best = std::i32::MIN;
        let mut best_pos = vec![];
        for (nex, pos) in board.list_next_with_pos() {
            let s = -self.search(&nex);
            if best < s {
                best = s;
                best_pos = pos;
            }
        }
        println!("{} {}", best, self.network.len());
        best_pos
    }
}
