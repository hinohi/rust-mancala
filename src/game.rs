use std::time::Instant;

use super::Searcher;
use crate::board::{Board, Side};

pub struct Game {
    board: Board,
    turn: usize,
    show_board: bool,
    ai_a: Box<dyn Searcher>,
    ai_b: Box<dyn Searcher>,
}

impl Game {
    pub fn new(stealing: bool, ai_a: Box<dyn Searcher>, ai_b: Box<dyn Searcher>) -> Game {
        Game {
            board: Board::new(stealing),
            turn: 0,
            show_board: false,
            ai_a,
            ai_b,
        }
    }

    pub fn show_board(&mut self, show: bool) {
        self.show_board = show;
    }

    pub fn first_sow(&mut self, pos_list: &[usize]) {
        for pos in pos_list {
            self.board.sow(*pos);
        }
    }

    fn proceed(&mut self) {
        let time = Instant::now();
        let pos_list = if self.board.side() == Side::First {
            self.ai_a.sow(&self.board)
        } else {
            self.ai_b.sow(&self.board)
        };
        let side = self.board.side();
        for &pos in pos_list.iter() {
            assert!(self.board.can_sow(pos).is_ok());
            assert_eq!(self.board.side(), side);
            self.board.sow(pos);
        }
        if self.show_board {
            println!("{:?} ({}ms)", pos_list, time.elapsed().as_millis());
            println!("{}", self.board);
        }
        self.turn += 1;
    }

    pub fn run(&mut self) -> (u8, u8) {
        if self.show_board {
            println!("{}", self.board);
        }
        loop {
            if self.board.is_finished() {
                let (a, b) = self.board.last_scores();
                return (a, b);
            }
            self.proceed();
        }
    }
}
