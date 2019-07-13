use std::env::args;

use mancala_rust::*;

fn main() {
    let list = [
        "random",
        "dfs:mc-10:1",
        "dfs:mc-10:2",
        "dfs:mc-10:3",
        "dfs:mc-10:4",
        "dfs:mc-100:1",
        "dfs:mc-100:2",
        "dfs:mc-100:3",
        "dfs:mc-1000:1",
        "dfs:mc-1000:2",
        "dfs:diff:7",
        "dfs:diff:9",
        "mctree:10",
        "mctree:14",
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
