use std::env::args;

use mancala_rust::*;

fn main() {
    let list = ["mctree:800:2:2", "random", "dfs:nn6:4"];
    let args = args().collect::<Vec<_>>();
    let stealing = args[1].parse().unwrap();
    loop {
        for &a in list.iter() {
            for &b in list.iter() {
                let mut game = Game::new(
                    stealing,
                    build_ai(stealing, a).unwrap(),
                    build_ai(stealing, b).unwrap(),
                );
                let (sa, sb) = game.run();
                println!("{a} {b} {sa} {sb}");
            }
        }
    }
}
