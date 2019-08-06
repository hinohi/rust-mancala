use mancala_rust::learn::*;

type Pos1Map = [[f64; 32]; 12];

fn count_pos1(stealing: bool) -> (Pos1Map, Pos1Map) {
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
        error[depth as usize] += 0.5 * (f64::from(exact) - predict).powi(2);
        count[depth as usize] += 1.0;
    }
    print_error(&error, &count);
}

fn nn4_check(model: &str, stealing: bool) {
    use ndarray::Array1;
    use rust_nn::predict::NN4Regression;
    use std::fs::File;
    use std::io::BufReader;

    let mut f = BufReader::new(File::open(model).unwrap());
    let mut nn = NN4Regression::new(&mut f);
    let mut error = [0.0; 40];
    let mut count = [0.0; 40];
    let mut input = Array1::zeros(12);
    for (seeds, exact, depth) in iter_load(stealing).unwrap() {
        for (x, &s) in input.iter_mut().zip(seeds.iter()) {
            *x = f64::from(s);
        }
        let predict = nn.predict(&input);
        error[depth as usize] += 0.5 * (f64::from(exact) - predict).powi(2);
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
        Some("pos1_check") => count_pos1_and_check(stealing),
        Some("nn4") => nn4_check(&args[2], stealing),
        _ => eprintln!("Usage: pos1_(check|show)|nn4"),
    }
}
