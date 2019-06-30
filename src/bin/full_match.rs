use mancala_rust::*;
use rand_pcg::Mcg128Xsl64;

fn ai_factory(s: &str) -> Box<AI> {
    let s_list = s.split(':').collect::<Vec<_>>();
    if s_list[0] == "random" {
        return Box::new(RandomAI::new(Mcg128Xsl64::new(1)));
    } else if s_list[0] == "depth" {
        let depth = s_list[2].parse().unwrap();
        return Box::new(DepthSearchAI::new(ScoreDiffEvaluation::new(), depth));
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
