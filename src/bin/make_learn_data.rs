use fnv::FnvHashMap;
use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use mancala_rust::{from_compact_key, learn::*, MCTree, AI};

fn main() {
    let stealing = true;
    let mut ai = MCTree::new(256, Mcg128Xsl64::from_entropy());
    let mut data = load(stealing);
    for i in 1..=30_000_000 {
        let mut path = to_finish(stealing, &mut ai);
        while let Some(board) = path.pop() {
            if search(&mut data, board, 20).is_none() {
                break;
            }
        }
        if i % 100 == 0 {
            println!("{} {}", i, data.len());
        }
    }

    println!("save: {:?}", save(stealing, &data));

    println!("depth histogram");
    let mut hist = [0; 256];
    for (key, (score, depth)) in data.iter() {
        hist[*depth as usize] += 1;
        if *depth > 30 {
            println!("{:?} depth={} {}", from_compact_key(*key), depth, score);
        }
    }
    for (depth, count) in hist.iter().enumerate() {
        println!("{} {}", depth, count);
        if *count == 0 {
            break;
        }
    }
}
