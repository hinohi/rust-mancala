use criterion::{Criterion, criterion_group, criterion_main};

use mancala_rust::{
    Board, DepthSearcher, NeuralNet4Evaluator, NeuralNet6Evaluator, ScoreDiffEvaluator, Searcher,
};

fn dfs5_with_stealing(c: &mut Criterion) {
    let mut ai = DepthSearcher::new(ScoreDiffEvaluator::new(), 5);
    c.bench_function("dfs5_with_stealing", |b| {
        b.iter(|| ai.sow(&Board::new(true)))
    });
}

fn dfs5_no_stealing(c: &mut Criterion) {
    let mut ai = DepthSearcher::new(ScoreDiffEvaluator::new(), 5);
    c.bench_function("dfs5_no_stealing", |b| {
        b.iter(|| ai.sow(&Board::new(false)))
    });
}

fn dfs6_with_stealing(c: &mut Criterion) {
    let mut ai = DepthSearcher::new(ScoreDiffEvaluator::new(), 6);
    c.bench_function("dfs6_with_stealing", |b| {
        b.iter(|| ai.sow(&Board::new(true)))
    });
}

fn dfs6_no_stealing(c: &mut Criterion) {
    let mut ai = DepthSearcher::new(ScoreDiffEvaluator::new(), 6);
    c.bench_function("dfs6_no_stealing", |b| {
        b.iter(|| ai.sow(&Board::new(false)))
    });
}

fn nn4_dfs2(c: &mut Criterion) {
    let mut ai = DepthSearcher::new(NeuralNet4Evaluator::new(true), 2);
    c.bench_function("nn4_dfs2", |b| b.iter(|| ai.sow(&Board::new(true))));
}

fn nn6_dfs2(c: &mut Criterion) {
    let mut ai = DepthSearcher::new(NeuralNet6Evaluator::new(true), 2);
    c.bench_function("nn6_dfs2", |b| b.iter(|| ai.sow(&Board::new(true))));
}

criterion_group!(
    benches,
    dfs5_with_stealing,
    dfs5_no_stealing,
    dfs6_with_stealing,
    dfs6_no_stealing,
    nn4_dfs2,
    nn6_dfs2,
);
criterion_main!(benches);
