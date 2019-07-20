use std::env::args;

use mancala_rust::*;

fn main() {
    let list = [
        "dfs:pos:3",
        "dfs:pos:5",
        "dfs:pos:7",
        "random",
        "greedy",
        "dfs:diff:9",
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
