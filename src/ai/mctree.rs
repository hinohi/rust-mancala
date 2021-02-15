use std::time::{Duration, Instant};

use rand::Rng;

use super::{
    utils::{choice_with_weight, random_down},
    AI,
};
use crate::Board;

#[derive(Debug, Clone)]
struct Node {
    visited_count: u32,
    win_count: u32,
    board: Board,
    is_win: Option<bool>,
    children: Vec<Node>,
}

pub struct McTreeAI<R> {
    rng: R,
    limit: Duration,
    expansion_threshold: u32,
    c: f64,
}

impl Node {
    fn new(board: Board) -> Node {
        Node {
            visited_count: 0,
            win_count: 0,
            board,
            is_win: None,
            children: Vec::new(),
        }
    }
}

impl<R: Rng> McTreeAI<R> {
    pub fn new(rng: R, limit: u64, expansion_threshold: u32, c: f64) -> McTreeAI<R> {
        McTreeAI {
            rng,
            limit: Duration::from_millis(limit),
            expansion_threshold,
            c,
        }
    }

    fn choice_child(&mut self, log_total_count: f64, node: &Node) -> usize {
        let mut weight = Vec::with_capacity(node.children.len());
        for (i, child) in node.children.iter().enumerate() {
            if child.visited_count == 0 {
                return i;
            }
            let a = child.win_count as f64 / child.visited_count as f64;
            let b = self.c * (log_total_count / child.visited_count as f64).sqrt();
            weight.push(a + b);
        }
        choice_with_weight(&mut self.rng, &weight)
    }

    fn selection(&mut self, log_total_count: f64, node: &mut Node) -> bool {
        node.visited_count += 1;
        if let Some(win) = node.is_win {
            if win {
                node.win_count += 1;
            }
            return win;
        }
        if node.children.is_empty() {
            if node.board.is_finished() {
                // 引き分けも勝ちに入れとく
                return if node.board.score() <= 0 {
                    node.is_win = Some(true);
                    node.win_count += 1;
                    true
                } else {
                    node.is_win = Some(false);
                    false
                };
            }
            if node.visited_count <= self.expansion_threshold {
                let board = random_down(&mut self.rng, &node.board);
                return if board.side() != node.board.side() {
                    if board.score() >= 0 {
                        node.win_count += 1;
                        true
                    } else {
                        false
                    }
                } else {
                    if board.score() <= 0 {
                        node.win_count += 1;
                        true
                    } else {
                        false
                    }
                };
            }
        }
        if node.children.is_empty() {
            node.children
                .extend(node.board.list_next().into_iter().map(Node::new));
        }
        let i = self.choice_child(log_total_count, node);
        let win = !self.selection(log_total_count, &mut node.children[i]);
        if win {
            node.win_count += 1;
        }
        win
    }
}

impl<R: Rng> AI for McTreeAI<R> {
    fn sow(&mut self, board: &Board) -> Vec<usize> {
        let start = Instant::now();
        let next_with_pos = board.list_next_with_pos();
        if next_with_pos.is_empty() {
            return Vec::new();
        }
        let mut node = Node::new(board.clone());
        for board in next_with_pos.keys() {
            node.children.push(Node::new(board.clone()));
        }
        let mut total_count = 0;
        while start.elapsed() < self.limit {
            for _ in 0..1000 {
                total_count += 1;
                self.selection((total_count as f64).ln(), &mut node);
            }
        }
        let best = node
            .children
            .iter()
            .max_by(|x, y| x.visited_count.cmp(&y.visited_count))
            .unwrap();
        next_with_pos[&best.board].clone()
    }
}
