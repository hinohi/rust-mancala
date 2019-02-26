use mancala_rust::*;


fn main() {
    let a = InteractiveAI::new();
    let b = DepthSearchAI::new(3);
    let mut judge = Judge::new(a, b);
    let (s, a, b) = judge.run();
    println!("{:?} {} {}", s, a, b);
}
