mod ai;
mod game;

use ai::*;

fn main() {
    for _ in 0..10000 {
        let ai_a = RandomAI::new();
        let ai_b = RandomAI::new();
        let mut judge = Judge::new(Box::new(ai_a), Box::new(ai_b));
        println!("{:?}", judge.run());
    }
}
