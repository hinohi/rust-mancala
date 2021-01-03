use mancala_rust::*;

fn main() {
    let stealing = true;
    let a = build_ai(stealing, "dfs:nn4:3").unwrap();
    let b = build_ai(stealing, "dfs:nn4:3").unwrap();
    let mut game = Game::new(stealing, a, b);
    game.show_board(false);
    let (a, b) = game.run();
    println!("{} {}", a, b);
}
