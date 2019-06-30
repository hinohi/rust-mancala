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
    }
    eprintln!("Usage: {{AI Name}}[:{{Option}}]\ninput: {}", s);
    exit(1);
}

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("Usage: {} AI AI", args[0]);
        exit(1);
    }
    let a = ai_factory(args[1].clone());
    let b = ai_factory(args[2].clone());
    let mut judge = Judge::new(a, b);
    let (a, b) = judge.run();
    println!("{} {}", a, b);
}
