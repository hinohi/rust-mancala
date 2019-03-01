use mancala_rust::*;

fn ai_factory(s: &str) -> Box<AI> {
    let s_list = s.split(':').collect::<Vec<_>>();
    if s_list[0] == "random" {
        return Box::new(RandomAI::new());
    } else if s_list[0] == "depth" {
        let depth = s_list[2].parse().unwrap();
        if s_list[1] == "diff" {
            return Box::new(DepthSearchAI::new(ScoreDiffEvaluation::new(), depth));
        } else {
            return Box::new(DepthSearchAI::new(PotEvaluation::new(), depth));
        };
    } else if s_list[0] == "cut" {
        let width = s_list[2].parse().unwrap();
        let depth = s_list[3].parse().unwrap();
        if s_list[1] == "diff" {
            return Box::new(CutDepthAI::new(ScoreDiffEvaluation::new(), width, depth));
        } else {
            return Box::new(CutDepthAI::new(PotEvaluation::new(), width, depth));
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
                let mut j = Judge::new(ai_factory(a), ai_factory(b));
                let (sa, sb) = j.run();
                println!("{} {} {} {}", a, b, sa, sb);
            }
        }
    }
}
