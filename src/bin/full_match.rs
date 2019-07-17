use std::env::args;

use mancala_rust::*;

fn main() {
    let list = [
        "random",
        "greedy",
        "dfs:diff:3",
        "dfs:diff:5",
        "dfs:diff:7",
        "mctree:8",
        "mctree:10",
        "mctree:12",
    ];
    let args = args().collect::<Vec<_>>();
    let stealing = args[1].parse().unwrap();
    loop {
        for &a in list.iter() {
            for &b in list.iter() {
                let mut game = Game::new(stealing, build_ai(a).unwrap(), build_ai(b).unwrap());
                let (sa, sb) = game.run();
                println!("{} {} {} {}", a, b, sa, sb);
            }
        }
    }
}
