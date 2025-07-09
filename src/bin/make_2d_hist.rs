#![cfg(feature = "make_hist")]
use std::env::args;
use std::thread::spawn;

use crossbeam::channel::{Receiver, Sender, bounded, unbounded};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rand_pcg::Mcg128Xsl64;

use mancala_rust::{Board, NeuralNet4Evaluator, NeuralNet6Evaluator, ab_search, learn::*};
use rust_nn::Float;

#[derive(Debug, Clone, PartialEq)]
struct Hist {
    min_value: Float,
    max_value: Float,
    value_step: Float,
    total_count: usize,
    hist: Vec<Vec<usize>>,
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
            hist: vec![vec![0; n + 2]; n + 2],
        }
    }

    fn clip(&self, value: Float) -> usize {
        if value < self.min_value {
            0
        } else if self.max_value < value {
            self.hist.len() - 1
        } else {
            ((value - self.min_value) / self.value_step).floor() as usize + 1
        }
    }

    fn incr(&mut self, a: Float, b: Float) {
        let a = self.clip(a);
        let b = self.clip(b);
        self.hist[a][b] += 1;
        self.total_count += 1;
    }

    fn dump(&self) {
        let f = 1.0 / self.total_count as f64 / self.value_step / self.value_step;
        for (i, row) in self.hist.iter().enumerate() {
            for (j, h) in row.iter().enumerate() {
                println!(
                    "{} {} {}",
                    self.min_value + self.value_step * (i as Float - 0.5),
                    self.min_value + self.value_step * (j as Float - 0.5),
                    *h as f64 * f,
                );
            }
            println!();
        }
    }
}

fn worker(stealing: bool, depth: usize, r: Receiver<(Board, i8)>, s: Sender<(Float, Float)>) {
    let mut eval4 = NeuralNet4Evaluator::new(stealing);
    let mut eval6 = NeuralNet6Evaluator::new(stealing);
    while let Ok((board, exact)) = r.recv() {
        let score4 = ab_search(board.clone(), &mut eval4, depth, -1e10, 1e10);
        let score6 = ab_search(board, &mut eval6, depth, -1e10, 1e10);
        s.send((score4 - exact as Float, score6 - exact as Float))
            .unwrap();
    }
}

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let stealing = args[0].parse().expect("stealing");
    let depth = args[1].parse().expect("depth");
    let db_path = args[2].clone();
    let num_worker = args[3].parse().expect("num worker");
    let use_rate = match args.get(4) {
        Some(s) => s.parse().unwrap(),
        None => 1.0,
    };
    let (board_s, board_r) = bounded(1024);
    let (score_s, score_r) = unbounded();

    for _ in 0..num_worker {
        let board_r = board_r.clone();
        let score_s = score_s.clone();
        spawn(move || worker(stealing, depth, board_r, score_s));
    }
    drop(score_s);

    spawn(move || {
        let db = iter_load(db_path).expect("DBが開けません");
        let n = db.size_hint().1.unwrap();
        let bar = ProgressBar::new(n as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{bar:40.cyan/blue} {pos}/{len} [{elapsed_precise}/{eta_precise}]")
                .unwrap(),
        );
        let mut r = Mcg128Xsl64::new(1);
        for (i, (seeds, exact, _)) in db.enumerate() {
            if (i + 1) % 1048576 == 0 {
                bar.inc(1048576);
            }
            if use_rate >= 1.0 || r.random_range(0.0..1.0) < use_rate {
                board_s
                    .send((Board::from_seeds(stealing, &seeds), exact))
                    .unwrap();
            }
        }
        bar.finish();
    });
    let h = spawn(move || {
        let mut hist = Hist::new(-20.0, 20.0, 2f64.powi(-3));
        while let Ok((a, b)) = score_r.recv() {
            hist.incr(a, b);
        }
        hist
    });
    let hist = h.join().unwrap();
    hist.dump();
}
