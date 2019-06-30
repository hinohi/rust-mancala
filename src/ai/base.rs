use crate::game::*;

pub trait AI {
    fn sow(&mut self, board: &Board) -> Vec<usize>;
}

pub struct Judge {
    board: Board,
    turn: usize,
    ai_a: Box<AI>,
    ai_b: Box<AI>,
}

impl Judge {
    pub fn new(ai_a: Box<AI>, ai_b: Box<AI>) -> Judge {
        Judge {
            board: Board::new(),
            turn: 0,
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

    pub fn run(&mut self) -> (u8, u8) {
        loop {
            if self.board.is_finished() {
                let (a, b) = self.board.last_scores();
                return (a, b);
            }
            self.proceed();
        }
    }
}
