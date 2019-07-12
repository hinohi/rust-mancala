use std::env::args;
use std::process::exit;

use mancala_rust::*;
use rand_pcg::Mcg128Xsl64;

fn ai_factory(s: String) -> Box<AI> {
    let s_list = s.split(':').collect::<Vec<_>>();
    if s_list[0] == "random" {
        return Box::new(RandomAI::new(Mcg128Xsl64::new(1)));
    } else if s_list[0] == "human" {
        return Box::new(InteractiveAI::new());
    } else if s_list[0] == "depth" {
        if s_list.len() < 2 {
            eprintln!("Usage: depth:(depth)");
            exit(1);
        }
        let depth = match s_list[1].parse() {
            Ok(depth) => depth,
            Err(e) => {
                eprintln!("parse fail: {}", e);
                exit(1);
            }
        };
        return Box::new(DepthSearchAI::new(ScoreDiffEvaluation::new(), depth));
    } else if s_list[0] == "mctree" {
        if s_list.len() < 2 {
            eprintln!("Usage: mctree:(num)");
            exit(1);
        }
        let path = match s_list[1].parse() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("parse fail: {}", e);
                exit(1);
            }
        };
        return Box::new(MCTree::new(path, Mcg128Xsl64::new(1)));
    } else if s_list[0] == "mctree_lf" {
        if s_list.len() < 3 {
            eprintln!("Usage: mctree_lf:(path):(full)");
            exit(1);
        }
        let path = match s_list[1].parse() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("parse fail: {}", e);
                exit(1);
            }
        };
        let full = match s_list[2].parse() {
            Ok(full) => full,
            Err(e) => {
                eprintln!("parse fail: {}", e);
                exit(1);
            }
        };
        return Box::new(MCTreeLF::new(path, full, Mcg128Xsl64::new(1)));
    } else if s_list[0] == "learned" {
        if s_list.len() < 2 {
            eprintln!("Usage: learned:(path)");
            exit(1);
        }
        let path = match s_list[1].parse() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("parse fail: {}", e);
                exit(1);
            }
        };
        let mut ai = LearnedMCTree::new(path, Mcg128Xsl64::new(1));
        ai.show_hit(true);
        return Box::new(ai);
    }
    eprintln!("Usage: {{AI Name}}[:{{Option}}]\ninput: {}", s);
    exit(1);
}

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() < 4 {
        eprintln!("Usage: {} STEAL AI AI", args[0]);
        exit(1);
    }
    let stealing = match args[1].parse() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
    let a = ai_factory(args[2].clone());
    let b = ai_factory(args[3].clone());
    let mut judge = Judge::new(stealing, a, b);
    judge.show_board(true);
    let (a, b) = judge.run();
    println!("{} {}", a, b);
}
