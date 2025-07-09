use std::env::args;

use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use mancala_rust::{RandomSearcher, learn::*};

fn main() {
    let stealing = args().nth(1).expect("USAGE: <stealing>").parse().unwrap();
    let mut ai = RandomSearcher::new(Mcg128Xsl64::from_rng(&mut rand::rng()));
    let mut data = load(&db_name(stealing));
    for i in 1..=30_000 {
        let mut path = to_finish(stealing, &mut ai);
        while let Some(board) = path.pop() {
            if search(&mut data, board, 30).is_none() {
                break;
            }
        }
        if i % 100 == 0 {
            println!("{} {}", i, data.len());
        }
    }

    println!("save: {:?}", save(&db_name(stealing), &data));

    println!("depth histogram");
    let mut hist = [0; 256];
    for (_, (_, depth)) in data.iter() {
        hist[*depth as usize] += 1;
    }
    for (depth, count) in hist.iter().enumerate() {
        println!("{depth} {count}");
        if *count == 0 {
            break;
        }
    }
}
