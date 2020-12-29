use criterion::{criterion_group, criterion_main, Criterion};

use rand_pcg::Mcg128Xsl64;

use mancala_rust::{
    Board, DepthSearchAI, MCTree, NN4Evaluator, NN6Evaluator, ScoreDiffEvaluator, AI,
};

fn mctree_with_stealing(c: &mut Criterion) {
    let mut ai = MCTree::new(10, Mcg128Xsl64::new(1));
    c.bench_function("mctree_with_stealing", |b| {
        b.iter(|| ai.sow(&Board::new(true)))
    });
}

fn mctree_no_stealing(c: &mut Criterion) {
    let mut ai = MCTree::new(10, Mcg128Xsl64::new(1));
    c.bench_function("mctree_no_stealing", |b| {
        b.iter(|| ai.sow(&Board::new(false)))
    });
}

fn dfs5_with_stealing(c: &mut Criterion) {
    let mut ai = DepthSearchAI::new(ScoreDiffEvaluator::new(), 5);
    c.bench_function("dfs5_with_stealing", |b| {
        b.iter(|| ai.sow(&Board::new(true)))
    });
}

fn dfs5_no_stealing(c: &mut Criterion) {
    let mut ai = DepthSearchAI::new(ScoreDiffEvaluator::new(), 5);
    c.bench_function("dfs5_no_stealing", |b| {
        b.iter(|| ai.sow(&Board::new(false)))
    });
}

fn dfs6_with_stealing(c: &mut Criterion) {
    let mut ai = DepthSearchAI::new(ScoreDiffEvaluator::new(), 6);
    c.bench_function("dfs6_with_stealing", |b| {
        b.iter(|| ai.sow(&Board::new(true)))
    });
}

fn dfs6_no_stealing(c: &mut Criterion) {
    let mut ai = DepthSearchAI::new(ScoreDiffEvaluator::new(), 6);
    c.bench_function("dfs6_no_stealing", |b| {
        b.iter(|| ai.sow(&Board::new(false)))
    });
}

fn nn4_dfs2(c: &mut Criterion) {
    let mut ai = DepthSearchAI::new(NN4Evaluator::new(true), 2);
    c.bench_function("nn4_dfs2", |b| b.iter(|| ai.sow(&Board::new(true))));
}

fn nn6_dfs2(c: &mut Criterion) {
    let mut ai = DepthSearchAI::new(NN6Evaluator::new(true), 2);
    c.bench_function("nn6_dfs2", |b| b.iter(|| ai.sow(&Board::new(true))));
}

criterion_group!(
    benches,
    mctree_with_stealing,
    mctree_no_stealing,
    dfs5_with_stealing,
    dfs5_no_stealing,
    dfs6_with_stealing,
    dfs6_no_stealing,
    nn4_dfs2,
    nn6_dfs2,
);
criterion_main!(benches);
