mod ai;
mod game;

use ai::*;

fn factory(side: usize, cmd: &str) -> Box<AI> {
    if cmd == "interactive" {
        return Box::new(InteractiveAI::new(side));
    } else if cmd == "random" {
        return Box::new(RandomAI::new(side));
    } else if cmd.starts_with("depth") {
        let depth: u32 = cmd[5..].parse().unwrap();
        return Box::new(DepthSearchAI::new(side, depth));
    }
    panic!("unknown AI: {}", cmd);
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        panic!("Usage: ./cmd N AI_a AI_b");
    }
    let n = args[0].parse().unwrap();
    let cmd_a = &args[1];
    let cmd_b = &args[2];
    for _ in 0..n {
        let ai_a = factory(0, &cmd_a);
        let ai_b = factory(1, &cmd_b);
        let mut judge = Judge::new(ai_a, ai_b);
        println!("{:?}", judge.run());
    }
}
