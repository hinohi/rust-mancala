use mancala_rust::learn::*;

fn count_pos1(stealing: bool) {
    let mut score = [[0.0; 32]; 12];
    let mut count = [[0.0; 32]; 12];
    for (seeds, s, _) in iter_load(stealing).unwrap() {
        let s = f64::from(s);
        for (pos, seed) in seeds.iter().enumerate() {
            let seed = *seed as usize;
            score[pos][seed] += s;
            count[pos][seed] += 1.0;
        }
    }
    for pos in 0..12 {
        for i in 0..32 {
            if count[pos][i] < 3.0 {
                println!("0");
            } else {
                println!("{}", score[pos][i] / count[pos][i] / 12.0);
            }
        }
    }
}

fn count_pos2(stealing: bool) {
    let mut score = [[[[0.0; 32]; 12]; 32]; 12];
    let mut count = [[[[0.0; 32]; 12]; 32]; 12];
    for (seeds, s, _) in iter_load(stealing).unwrap() {
        let s = f64::from(s);
        for (p1, s1) in seeds.iter().enumerate() {
            let s1 = *s1 as usize;
            for (p2, s2) in seeds.iter().enumerate() {
                let s2 = *s2 as usize;
                score[p1][s1][p2][s2] += s;
                count[p1][s1][p2][s2] += 1.0;
            }
        }
    }
    for p1 in 0..12 {
        for s1 in 0..32 {
            for p2 in 0..12 {
                for s2 in 0..32 {
                    if count[p1][s1][p2][s2] < 3.0 {
                        println!("0");
                    } else {
                        println!("{}", score[p1][s1][p2][s2] / count[p1][s1][p2][s2] / 144.0);
                    }
                }
            }
        }
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
