use mancala_rust::learn::*;

fn count_pos1(stealing: bool) {
    let mut score = [[0; 32]; 12];
    let mut count = [[0; 32]; 12];
    for (seeds, s, _) in iter_load(stealing).unwrap() {
        for (pos, seed) in seeds.iter().enumerate() {
            score[pos][*seed as usize] += s as i64;
            count[pos][*seed as usize] += 1_i64;
        }
    }
    for pos in 0..12 {
        print!("[");
        for i in 0..32 {
            if count[pos][i] < 100 {
                print!("0.0, ");
            } else {
                print!(
                    "{:.4}, ",
                    score[pos][i] as f64 / count[pos][i] as f64 / 12.0
                );
            }
        }
        println!("],");
    }
}

fn count_pos2(stealing: bool) {
    let mut score = [[[[0; 32]; 12]; 32]; 12];
    let mut count = [[[[0; 32]; 12]; 32]; 12];
    for (seeds, s, _) in iter_load(stealing).unwrap() {
        for (p1, s1) in seeds.iter().enumerate() {
            let s1 = *s1 as usize;
            for (p2, s2) in seeds.iter().enumerate() {
                let s2 = *s2 as usize;
                score[p1][s1][p2][s2] += s as i64;
                count[p1][s1][p2][s2] += 1_u64;
            }
        }
    }
    for p1 in 0..12 {
        println!("[");
        for s1 in 0..32 {
            for p2 in 0..12 {
                print!("    [");
                for s2 in 0..32 {
                    if count[p1][s1][p2][s2] < 10 {
                        print!("0.0, ");
                    } else {
                        print!(
                            "{}, ",
                            score[p1][s1][p2][s2] as f64 / count[p1][s1][p2][s2] as f64 / 144.0
                        );
                    }
                }
                println!("],")
            }
        }
        println!("],");
    }
}

fn main() {
    let stealing = true;
    let args = std::env::args().collect::<Vec<_>>();
    if args[1].parse::<i32>().unwrap() == 1 {
        count_pos1(stealing);
    } else {
        count_pos2(stealing);
    }
}
