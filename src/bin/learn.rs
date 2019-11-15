use std::fs::File;
use std::io::BufWriter;

use ndarray::Array2;
use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use mancala_rust::learn::{RepeatLod, ShuffledStream};
use rust_nn::{train::*, Float};

const ONE_HOT_MAX: usize = 31;

fn gen_case<I>(x: &mut Array2<Float>, t: &mut Array2<Float>, data: &mut I)
where
    I: Iterator<Item = ([u8; 12], i8, u8)>,
{
    x.fill(0.0);
    for (mut x, mut t) in x.genrows_mut().into_iter().zip(t.genrows_mut()) {
        let (board, score, _) = match data.next() {
            Some(row) => row,
            None => return,
        };
        for (i, &b) in board.iter().enumerate() {
            x[i * ONE_HOT_MAX + b as usize] = 1.0;
        }
        t[0] = Float::from(score);
    }
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let db_path = args[0].as_str();
    let save_path = args[1].as_str();
    assert_ne!(db_path, save_path);
    let batch_size = 128;
    let mut model = NN2Regression::new(
        [12 * ONE_HOT_MAX, 128, 128],
        batch_size,
        Adam::default(),
        Adam::default(),
    );

    let mut x = Array2::zeros([batch_size, 12 * ONE_HOT_MAX]);
    let mut t = Array2::zeros([batch_size, 1]);
    let mut data = ShuffledStream::new(
        RepeatLod::new(db_path),
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
            let mut f = BufWriter::new(File::create(&save_path).unwrap());
            model.encode(&mut f);
        }
    }
}
