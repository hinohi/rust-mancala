use std::io::{stdin, stdout, Write};
use std::string::String;

extern crate mancala_rust;
use mancala_rust::game::*;

fn interactive_game() {
    let mut board = Board::new();
    while board.get_state() == GameState::InBattle {
        println!("{}", board);
        let pos: usize;
        loop {
            print!("your turn: ");
            stdout().flush().unwrap();
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            match buf.trim().parse() {
                Ok(i) => match board.check_pos(i) {
                    Ok(_) => {
                        pos = i;
                        break;
                    }
                    Err(e) => println!("{}", e),
                },
                Err(e) => println!("{}", e),
            }
        }
        board.move_one(pos);
    }
}

fn main() {
    let board = Board::new();
    for (nex_board, pos_list) in board.list_next() {
        println!("{:?}", pos_list);
        println!("{}", nex_board);
    }
}
