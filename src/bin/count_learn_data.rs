use mancala_rust::learn::*;

fn main() {
    let stealing = true;
    let mut score = [[0; 32]; 12];
    let mut count = [[0; 32]; 12];
    for (seeds, s, d) in iter_load(stealing).unwrap() {
        if d < 2 {
            continue;
        }
        for (pos, seed) in seeds.iter().enumerate() {
            score[pos][*seed as usize] += s as i64;
            count[pos][*seed as usize] += 1_i64;
        }
    }
    for pos in 0..12 {
        print!("[");
        for i in 0..32 {
            if count[pos][i] == 0 {
                print!("0.0, ");
            } else {
                print!("{:.2}, ", score[pos][i] as f64 / count[pos][i] as f64);
            }
        }
        println!("],");
    }
}
