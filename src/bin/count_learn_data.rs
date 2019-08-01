use mancala_rust::learn::*;

fn count_pos1(stealing: bool) -> ([[f64; 32]; 12], [[f64; 32]; 12]) {
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
    (score, count)
}

fn count_pos1_and_show(stealing: bool) {
    let (score, count) = count_pos1(stealing);
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

fn count_pos1_and_check(stealing: bool) {
    let (score, count) = count_pos1(stealing);
    let mut map = [[0.0; 32]; 12];
    for pos in 0..12 {
        for i in 0..32 {
            if count[pos][i] > 0.0 {
                map[pos][i] = score[pos][i] / count[pos][i] / 12.0;
            }
        }
    }
    let mut error = [0.0; 40];
    let mut count = [0.0; 40];
    for (seeds, exact, depth) in iter_load(stealing).unwrap() {
        let mut predict = 0.0;
        for (pos, &s) in seeds.iter().enumerate() {
            predict += map[pos][s as usize];
        }
        error[depth as usize] += 0.5 * (exact as f64 - predict).powi(2);
        count[depth as usize] += 1.0;
    }
    print_error(&error, &count);
}

fn count_pos2(stealing: bool) -> ([[[[f64; 32]; 12]; 32]; 12], [[[[f64; 32]; 12]; 32]; 12]) {
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
    (score, count)
}

fn count_pos2_and_show(stealing: bool) {
    let (score, count) = count_pos2(stealing);
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

fn count_pos2_and_check(stealing: bool) {
    let (score, count) = count_pos2(stealing);
    let mut map = [[[[0.0; 32]; 12]; 32]; 12];
    for p1 in 0..12 {
        for s1 in 0..32 {
            for p2 in 0..12 {
                for s2 in 0..32 {
                    if count[p1][s1][p2][s2] > 0.0 {
                        map[p1][s1][p2][s2] = score[p1][s1][p2][s2] / count[p1][s1][p2][s2] / 144.0;
                    }
                }
            }
        }
    }
    let mut error = [0.0; 40];
    let mut count = [0.0; 40];
    for (seeds, exact, depth) in iter_load(stealing).unwrap() {
        let mut predict = 0.0;
        for (p1, &s1) in seeds.iter().enumerate() {
            for (p2, &s2) in seeds.iter().enumerate() {
                predict += map[p1][s1 as usize][p2][s2 as usize];
            }
        }
        error[depth as usize] += 0.5 * (exact as f64 - predict).powi(2);
        count[depth as usize] += 1.0;
    }
    print_error(&error, &count);
}

fn print_error(error: &[f64], count: &[f64]) {
    let mut te = 0.0;
    let mut tc = 0.0;
    for (depth, (&e, &c)) in error.iter().zip(count.iter()).enumerate() {
        if c > 0.0 {
            println!("{} {} ({})", depth, e / c, c);
        }
        te += e;
        tc += c;
    }
    println!("{}", te / tc);
}

fn main() {
    let stealing = true;
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).and_then(|s| Some(s.as_str())) {
        Some("pos1_show") => count_pos1_and_show(stealing),
        Some("pos2_show") => count_pos2_and_show(stealing),
        Some("pos1_check") => count_pos1_and_check(stealing),
        Some("pos2_check") => count_pos2_and_check(stealing),
        _ => eprintln!("Usage: (pos1|pos2)_(check|show)"),
    }
}
