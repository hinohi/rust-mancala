use std::env::args;
use std::thread::spawn;

use crossbeam::channel::{bounded, unbounded, Receiver, Sender};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rand_pcg::Mcg128Xsl64;

use mancala_rust::{ab_search, learn::*, Board, NN6Evaluator};
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
                *h as f64 / self.total_count as f64 / self.value_step,
            );
        }
    }
}

fn worker(stealing: bool, depth: usize, r: Receiver<(Board, i8)>, s: Sender<Float>) {
    let mut eval = NN6Evaluator::new(stealing);
    while let Ok((board, exact)) = r.recv() {
        let score = ab_search(board, &mut eval, depth, -1e10, 1e10);
        s.send(score - exact as Float).unwrap();
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
                .template("{bar:40.cyan/blue} {pos}/{len} [{elapsed_precise}/{eta_precise}]"),
        );
        let mut r = Mcg128Xsl64::new(1);
        for (i, (seeds, exact, _)) in db.enumerate() {
            if (i + 1) % 1048576 == 0 {
                bar.inc(1048576);
            }
            if use_rate >= 1.0 || r.gen_range(0.0, 1.0) < use_rate {
                board_s
                    .send((Board::from_seeds(stealing, &seeds), exact))
                    .unwrap();
            }
        }
        bar.finish();
    });
    let h = spawn(move || {
        let mut hist = Hist::new(-50.0, 50.0, 2f64.powi(-10));
        while let Ok(diff) = score_r.recv() {
            hist.count(diff);
        }
        hist
    });
    let hist = h.join().unwrap();
    hist.dump();
}
