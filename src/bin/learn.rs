use std::fs::File;
use std::io::BufWriter;

use ndarray::arr2;

use mancala_rust::learn::iter_load;
use rust_nn::train::{Layer, NN3Regression};
use std::process::exit;

fn main() {
    let stealing = true;
    let batch_size = 100;
    let mut model = NN3Regression::new([12, 16, 16, 16], batch_size, 1e-4);

    let mut x = Vec::new();
    let mut t = Vec::new();
    for _ in 0..batch_size {
        x.push([0.0; 12]);
        t.push([0.0; 1]);
    }
    let mut x = arr2(&x);
    let mut t = arr2(&t);

    let mut epoch = 0_u64;
    let mut loss = 0.0;
    let mut data = iter_load(stealing).unwrap();
    'OUT: loop {
        let (board, score, _) = match data.next() {
            Some(row) => row,
            None => exit(0),
        };
        for (mut x, mut t) in x.genrows_mut().into_iter().zip(t.genrows_mut()) {
            for (x, b) in x.iter_mut().zip(board.iter()) {
                *x = f64::from(*b);
            }
            t[0] = f64::from(score);
        }

        loss += model.train(&x, &t);
        epoch += 1;
        if epoch % 1000 == 0 {
            println!("{} {}", epoch, loss / 1000.0);
            loss = 0.0;
        }
        if epoch % 10_000_000 == 0 {
            let name = format!("model/NN3_16_{}.model", epoch);
            let mut f = BufWriter::new(File::create(name).unwrap());
            model.get_inner().encode(&mut f);
        }
    }
}
