use std::fs::File;
use std::io::{BufReader, BufWriter};

use ndarray::{arr2, Array2};

use mancala_rust::learn::iter_load;
use rust_nn::train::*;
use rust_nn::Float;

fn gen_case<I>(x: &mut Array2<Float>, t: &mut Array2<Float>, data: &mut I) -> bool
where
    I: Iterator<Item = ([u8; 12], i8, u8)>,
{
    for (mut x, mut t) in x.genrows_mut().into_iter().zip(t.genrows_mut()) {
        let (board, score, _) = match data.next() {
            Some(row) => row,
            None => return false,
        };
        for (x, b) in x.iter_mut().zip(board.iter()) {
            *x = f64::from(*b);
        }
        t[0] = f64::from(score);
    }
    true
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let stealing = true;
    let batch_size = 128;
    let pow2 = match args.get(0) {
        Some(n) => n.parse::<i32>().unwrap(),
        None => 13, // 1.220703125e-4
    };
    let lr = 2f64.powi(-pow2);
    let mut model = match args.get(1) {
        None => NN4Regression::new(
            [12, 64, 64, 64, 64],
            batch_size,
            SGD::default().learning_rate(lr),
            SGD::default().learning_rate(lr),
        ),
        Some(path) => {
            let mut f = BufReader::new(File::open(path).unwrap());
            NN4Regression::decode(
                &mut f,
                batch_size,
                SGD::default().learning_rate(lr),
                SGD::default().learning_rate(lr),
            )
        }
    };

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
    loop {
        let mut data = iter_load(stealing)
            .unwrap()
            .filter(|(_, _, depth)| *depth > 0);
        while gen_case(&mut x, &mut t, &mut data) {
            loss += model.train(&x, &t);
            epoch += 1;
            if epoch % 100 == 0 {
                println!("{} {}", epoch, loss / 100.0);
                loss = 0.0;
            }
            if epoch % 100_000 == 0 {
                let name = format!("model/NN4_{}_{}.model", pow2, epoch);
                let mut f = BufWriter::new(File::create(name).unwrap());
                model.encode(&mut f);
            }
        }
    }
}