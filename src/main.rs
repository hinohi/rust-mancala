#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate lazy_static;

mod ai;
mod game;
mod learn;

use ai::*;
use game::Board;
use learn::*;
use std::time::SystemTime;

fn factory(side: usize, cmd: &str) -> Box<AI> {
    if cmd == "interactive" {
        return Box::new(InteractiveAI::new(side));
    } else if cmd == "random" {
        return Box::new(RandomAI::new(side));
    } else if cmd.starts_with("depth") {
        let depth: u32 = cmd[5..].parse().unwrap();
        return Box::new(DepthSearchAI::new(side, depth));
    } else if cmd.starts_with("mem") {
        let depth: u32 = cmd[3..].parse().unwrap();
        return Box::new(MemAI::new(side, depth));
    }
    panic!("unknown AI: {}", cmd);
}

fn battle(args: &[String]) {
    let n = args[0].parse().unwrap();
    let cmd_a = &args[1];
    let cmd_b = &args[2];
    for _ in 0..n {
        let ai_a = factory(0, &cmd_a);
        let ai_b = factory(1, &cmd_b);
        let mut judge = Judge::new(ai_a, ai_b);
        let result = judge.run();
        println!("{:?} {} {}", result.0, result.1, result.2);
    }
}

fn learn(args: &[String]) {
    let start = SystemTime::now();
    let n = args[0].parse().unwrap();
    let mut s = Searcher::from_file(&args[1]);
    println!(
        "[{:5}] read done: {}",
        start.elapsed().unwrap().as_secs(),
        s.info()
    );
    for i in 0..n {
        s.single_run();
        if (i + 1) % (n / 10) == 0 {
            println!(
                "[{:5}] {}/{} done: {}",
                start.elapsed().unwrap().as_secs(),
                i + 1,
                n,
                s.info()
            );
        }
    }
    s.dump(&args[1]);
    println!("[{:5}] dump done", start.elapsed().unwrap().as_secs());
}

fn all_learn(args: &[String]) {
    let start = SystemTime::now();
    let mut s = Searcher::new();
    let board = Board::new();
    let score = s.search(&board, 1000);
    s.dump(&args[0]);
    println!("score={:?} size={}", score, s.info());
    println!("[{:5}] dump done", start.elapsed().unwrap().as_secs());
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        panic!(
            r#"Usage:
  ./cmd battle N AI_a AI_b
  ./cmd learn N
  ./cmd all
"#
        );
    }
    if args[0] == "battle" {
        battle(&args[1..]);
    } else if args[0] == "learn" {
        learn(&args[1..]);
    } else if args[0] == "all" {
        all_learn(&args[1..]);
    }
}
