use std::env::args;
use std::fs::File;

use ndarray::Array1;
use rand::{Rng, SeedableRng};
use rand_pcg::Mcg128Xsl64;

use mancala_rust::learn::*;
use rust_nn::predict::*;
use rust_nn::Float;

#[derive(Debug, Clone, PartialEq)]
struct Hist {
    min_value: Float,
    max_value: Float,
    value_step: Float,
    total_count: usize,
    hist: Vec<usize>,
}

impl Hist {
    fn new(min_value: Float, max_value: Float, value_step: Float) -> Hist {
        let n = ((max_value - min_value) / value_step).ceil() as usize;
        let value_step = (max_value - min_value) / n as Float;
        Hist {
            min_value,
            max_value,
            value_step,
            total_count: 0,
            hist: vec![0; n + 2],
        }
    }

    fn count(&mut self, value: Float) {
        if value < self.min_value {
            self.hist[0] += 1;
        } else if self.max_value < value {
            *self.hist.last_mut().unwrap() += 1;
        } else {
            let i = ((value - self.min_value) / self.value_step).floor() as usize;
            self.hist[i + 1] += 1;
        }
        self.total_count += 1;
    }

    fn dump(&self) {
        for (i, h) in self.hist.iter().enumerate() {
            println!(
                "{} {}",
                self.min_value + self.value_step * (i as Float - 0.5),
                *h as f64 / self.total_count as f64,
            );
        }
    }
}

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let mut model = {
        let mut f = std::io::BufReader::new(
            File::open(&args[0]).expect("モデルファイルが開けません"),
        );
        NN6Regression::new(&mut f)
    };
    let db = iter_load(&args[1]).expect("DBが開けません");
    let mut random = Mcg128Xsl64::from_entropy();
    let mut hist = Hist::new(-20.0, 20.0, 2f64.powi(-4));
    let mut arr = Array1::zeros(12);
    for (board, exact, _) in db {
        if random.gen_range(0.0, 1.0) < 1e-2 {
            for (i, &s) in board.iter().enumerate() {
                arr[i] = s as Float;
            }
            let predict = model.predict(&arr);
            hist.count(predict - exact as Float);
        }
    }
    hist.dump();
}
