use std::fs::File;
use std::io::{BufReader, BufWriter};

use ndarray::{arr2, Array2};
use rand::{Rng, SeedableRng};
use rand_pcg::Mcg128Xsl64;

use mancala_rust::learn::{db_name, iter_load, Load};
use rust_nn::train::*;
use rust_nn::Float;

struct ShuffledStream<I, R>
where
    I: Iterator,
{
    iter: I,
    random: R,
    buf: Vec<I::Item>,
}

impl<I, R> ShuffledStream<I, R>
where
    I: Iterator,
    R: Rng,
{
    fn new(iter: I, random: R, buffer: usize) -> ShuffledStream<I, R> {
        let mut iter = iter;
        let mut buf = Vec::with_capacity(buffer);
        for _ in 0..buffer {
            buf.push(iter.next().unwrap())
        }
        ShuffledStream { iter, random, buf }
    }
}

impl<I, R> Iterator for ShuffledStream<I, R>
where
    I: Iterator,
    R: Rng,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.random.gen_range(0, self.buf.len());
        let item = self.buf.swap_remove(idx);
        self.buf.push(self.iter.next().unwrap());
        Some(item)
    }
}

struct DataIter {
    stealing: bool,
    loader: Load,
}

impl DataIter {
    fn new(stealing: bool) -> DataIter {
        DataIter {
            stealing,
            loader: iter_load(&db_name(stealing)).unwrap(),
        }
    }
}

impl Iterator for DataIter {
    type Item = ([u8; 12], i8, u8);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.loader.next() {
                Some(item) => {
                    if item.2 > 0 {
                        return Some(item);
                    }
                }
                None => {
                    self.loader = iter_load(&db_name(self.stealing)).unwrap();
                }
            }
        }
    }
}

fn gen_case<I>(x: &mut Array2<Float>, t: &mut Array2<Float>, data: &mut I)
where
    I: Iterator<Item = ([u8; 12], i8, u8)>,
{
    for (mut x, mut t) in x.genrows_mut().into_iter().zip(t.genrows_mut()) {
        let (board, score, _) = match data.next() {
            Some(row) => row,
            None => return,
        };
        for (x, b) in x.iter_mut().zip(board.iter()) {
            *x = Float::from(*b);
        }
        t[0] = Float::from(score);
    }
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let stealing = args[0].parse().unwrap();
    let batch_size = 128;
    let pow2 = match args.get(1) {
        Some(n) => n.parse::<i32>().unwrap(),
        None => 13, // 1.220703125e-4
    };
    let lr = 2f64.powi(-pow2) as Float;
    let mut model = match args.get(2) {
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

    let mut data = ShuffledStream::new(
        DataIter::new(stealing),
        Mcg128Xsl64::from_entropy(),
        batch_size * 1024,
    );
    let mut epoch = 0_u64;
    let mut loss = 0.0;
    loop {
        gen_case(&mut x, &mut t, &mut data);
        loss += model.train(&x, &t);
        epoch += 1;
        if epoch % 1_000 == 0 {
            println!("{} {}", epoch, loss / 1000.0);
            loss = 0.0;
        }
        if epoch % 100_000 == 0 {
            let name = format!("model/NN4_{}_{}_{}.model", stealing, pow2, epoch);
            let mut f = BufWriter::new(File::create(name).unwrap());
            model.encode(&mut f);
        }
    }
}
