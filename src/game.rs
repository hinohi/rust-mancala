use super::AI;
use crate::board::*;

pub struct Game {
    board: Board,
    turn: usize,
    show_board: bool,
    ai_a: Box<dyn AI>,
    ai_b: Box<dyn AI>,
}

impl Game {
    pub fn new(stealing: bool, ai_a: Box<dyn AI>, ai_b: Box<dyn AI>) -> Game {
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

    fn proceed(&mut self) {
        let pos_list = if self.board.side == 0 {
            self.ai_a.sow(&self.board)
        } else {
            self.ai_b.sow(&self.board)
        };
        for &pos in pos_list.iter() {
            assert!(self.board.can_sow(pos).is_ok());
            self.board.sow(pos);
        }
        if self.show_board {
            println!("{:?}", pos_list);
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
