use mancala_rust::{from_compact_key, learn::*};

fn main() {
    let stealing = true;
    let data = load(stealing);

    let mut score = [[0; 32]; 12];
    let mut count = [[0; 32]; 12];
    for (key, (s, _)) in data.iter() {
        let seeds = from_compact_key(*key);
        for (pos, seed) in seeds.iter().enumerate() {
            score[pos][*seed as usize] += *s as i64;
            count[pos][*seed as usize] += 1_i64;
        }
    }
    for pos in 0..12 {
        for i in 0..32 {
            if count[pos][i] == 0 {
                print!("0 ");
            } else {
                print!("{:.1} ", score[pos][i] as f64 / count[pos][i] as f64);
            }
        }
        print!("\n");
    }
}
