use criterion::{Criterion, criterion_group, criterion_main};

use mancala_rust::Board;

fn list_next_with_stealing(c: &mut Criterion) {
    c.bench_function("list_next_with_stealing", |b| {
        b.iter(|| Board::new(true).list_next())
    });
}

fn list_next_no_stealing(c: &mut Criterion) {
    c.bench_function("list_next_no_stealing", |b| {
        b.iter(|| Board::new(false).list_next())
    });
}

fn list_next_with_pos_with_stealing(c: &mut Criterion) {
    c.bench_function("list_next_with_pos_with_stealing", |b| {
        b.iter(|| Board::new(true).list_next_with_pos())
    });
}

fn list_next_with_pos_no_stealing(c: &mut Criterion) {
    c.bench_function("list_next_with_pos_no_stealing", |b| {
        b.iter(|| Board::new(false).list_next_with_pos())
    });
}

criterion_group!(
    benches,
    list_next_with_stealing,
    list_next_no_stealing,
    list_next_with_pos_with_stealing,
    list_next_with_pos_no_stealing,
);
criterion_main!(benches);
