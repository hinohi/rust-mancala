use mancala_rust::*;

fn main() {
    let a = DepthSearchAI::new(5);
    let b = DepthSearchAI::new(6);
    let mut judge = Judge::new(a, b);
    let (s, a, b) = judge.run();
    println!("{:?} {} {}", s, a, b);
}
