use std::env::args;
use std::process::exit;

use mancala_rust::*;

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() < 4 {
        eprintln!("Usage: {} STEAL AI AI", args[0]);
        exit(1);
    }
    let stealing = match args[1].parse::<bool>() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    };
    let a = match build_ai(stealing, &args[2]) {
        Ok(ai) => ai,
        Err(e) => {
            eprintln!("Usage: {e}");
            exit(1);
        }
    };
    let b = match build_ai(stealing, &args[3]) {
        Ok(ai) => ai,
        Err(e) => {
            eprintln!("Usage: {e}");
            exit(1);
        }
    };
    let first_sow = args[4..]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let mut game = Game::new(stealing, a, b);
    game.show_board(true);
    game.first_sow(&first_sow);
    let (a, b) = game.run();
    println!("{a} {b}");
}
