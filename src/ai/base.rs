use crate::game::*;

pub trait AI {
    fn deliver(&mut self, board: &Board) -> (usize, usize, u8);
    fn sow(&mut self, board: &Board) -> Vec<usize>;
}

pub struct Judge {
    board: Board,
    turn: usize,
    deliver_num: usize,
    ai_a: Box<AI>,
    ai_b: Box<AI>,
}

impl Judge {
    pub fn new(deliver_num: usize, ai_a: Box<AI>, ai_b: Box<AI>) -> Judge {
        Judge {
            board: Board::new(),
            turn: 0,
            deliver_num,
            ai_a,
            ai_b,
        }
    }

    fn proceed(&mut self) {
        let pos_list;
        if self.board.side == 0 {
            pos_list = self.ai_a.sow(&self.board);
        } else {
            pos_list = self.ai_b.sow(&self.board);
        }
        for pos in pos_list {
            assert!(self.board.can_sow(pos).is_ok());
            self.board.sow(pos);
        }
        self.turn += 1;
    }

    pub fn run(&mut self) -> ([u8; PIT], u8, u8) {
        for _ in 0..self.deliver_num {
            let (pos_from, pos_to, num) = self.ai_b.deliver(&self.board);
            assert!(self.board.can_deliver(pos_from, pos_to, num));
            self.board.deliver(pos_from, pos_to, num);
        }
        let start_board = self.board.get_seeds().clone();
        loop {
            if self.board.is_finished() {
                let (a, b) = self.board.get_scores();
                return (start_board[0], a, b);
            }
            self.proceed();
        }
    }
}
