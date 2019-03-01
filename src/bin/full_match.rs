use mancala_rust::*;
use rand_pcg::Mcg128Xsl64;

fn ai_factory(s: &str) -> Box<AI> {
    let s_list = s.split(':').collect::<Vec<_>>();
    if s_list[0] == "random" {
        return Box::new(RandomAI::new(Mcg128Xsl64::new(1)));
    } else if s_list[0] == "depth" {
        let depth = s_list[2].parse().unwrap();
        if s_list[1] == "diff" {
            return Box::new(DepthSearchAI::new(ScoreDiffEvaluation::new(), depth));
        } else {
            return Box::new(DepthSearchAI::new(PotEvaluation::new(), depth));
        };
    }
    unreachable!();
}

fn main() {
    let list = [
        "random",
        "depth:diff:1",
        "depth:diff:2",
        "depth:diff:3",
        "depth:diff:4",
        "depth:diff:5",
        "depth:diff:6",
        "depth:pot:1",
        "depth:pot:2",
        "depth:pot:3",
        "depth:pot:4",
        "depth:pot:5",
        "depth:pot:6",
    ];
    loop {
        for &a in list.iter() {
            for &b in list.iter() {
                let mut j = Judge::new(1, ai_factory(a), ai_factory(b));
                let (board, sa, sb) = j.run();
                let board = board
                    .iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<_>>()
                    .join(",");
                println!("{} {} {} {} {}", board, a, b, sa, sb);
            }
        }
    }
}
