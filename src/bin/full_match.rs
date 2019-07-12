use mancala_rust::*;

fn main() {
    let list = ["random", "depth:7", "mctree:10", "mctree:12", "mctree:14"];
    loop {
        for &a in list.iter() {
            for &b in list.iter() {
                let mut game = Game::new(false, build_ai(a).unwrap(), build_ai(b).unwrap());
                let (sa, sb) = game.run();
                println!("{} {} {} {}", a, b, sa, sb);
            }
        }
    }
}
