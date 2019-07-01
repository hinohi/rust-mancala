use mancala_rust::*;
use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

fn ai_factory(s: &str) -> Box<AI> {
    let s_list = s.split(':').collect::<Vec<_>>();
    if s_list[0] == "random" {
        return Box::new(RandomAI::new(Mcg128Xsl64::from_entropy()));
    } else if s_list[0] == "depth" {
        let depth = s_list[2].parse().unwrap();
        return Box::new(DepthSearchAI::new(ScoreDiffEvaluation::new(), depth));
    } else if s_list[0] == "mctree" {
        let path = 1 << s_list[1].parse::<i32>().unwrap();
        return Box::new(MCTree::new(path as usize, Mcg128Xsl64::from_entropy()));
    } else if s_list[0] == "mctree_lf" {
        let path = 1 << s_list[1].parse::<i32>().unwrap();
        let full = s_list[2].parse::<u8>().unwrap();
        return Box::new(MCTreeLF::new(
            dbg!(path as usize),
            dbg!(full),
            Mcg128Xsl64::from_entropy(),
        ));
    }
    unreachable!();
}

fn main() {
    let list = ["random", "mctree:10", "mctree_lf:4:10"];
    loop {
        for &a in list.iter() {
            for &b in list.iter() {
                let mut j = Judge::new(false, ai_factory(a), ai_factory(b));
                let (sa, sb) = j.run();
                println!("{} {} {} {}", a, b, sa, sb);
            }
        }
    }
}
