use game::*;

pub trait AI {
    fn think(&mut self, board: &Board) -> Vec<usize>;
}

pub struct Judge {
    board: Board,
    turn: u32,
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

    fn proceed(&mut self) -> GameState {
        let pos_list;
        if self.board.side == 0 {
            pos_list = self.ai_a.think(&self.board);
        } else {
            pos_list = self.ai_b.think(&self.board);
        }
        for pos in pos_list {
            assert!(self.board.check_pos(pos).is_ok());
            self.board.move_one(pos);
        }
        self.turn += 1;
        self.board.get_state()
    }

    pub fn run(&mut self) -> (GameState, u8, u8) {
        if self.board.get_state() != GameState::InBattle {
            let s = self.board.get_scores();
            return (self.board.get_state(), s.0, s.1);
        }
        loop {
            let state = self.proceed();
            if state != GameState::InBattle {
                let s = self.board.get_scores();
                return (state, s.0, s.1);
            }
        }
    }
}
