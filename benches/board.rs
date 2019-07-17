#![feature(test)]

extern crate test;
use test::Bencher;

use mancala_rust::Board;

#[bench]
fn list_next_with_stealing(b: &mut Bencher) {
    b.iter(|| Board::new(true).list_next());
}

#[bench]
fn list_next_no_stealing(b: &mut Bencher) {
    b.iter(|| Board::new(false).list_next());
}

#[bench]
fn list_next_with_pos_with_stealing(b: &mut Bencher) {
    b.iter(|| Board::new(true).list_next_with_pos());
}

#[bench]
fn list_next_with_pos_no_stealing(b: &mut Bencher) {
    b.iter(|| Board::new(false).list_next_with_pos());
}
