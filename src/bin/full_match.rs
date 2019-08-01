use std::env::args;

use mancala_rust::*;

fn main() {
    let list = [
        "dfs:nn:5",
        "weighted:diff:10",
        "weighted:pos:10",
        "weighted:nn:10",
        "random",
        "dfs:diff:9",
        "dfs:pos:7",
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
