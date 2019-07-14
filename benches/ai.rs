#![feature(test)]

extern crate test;
use test::Bencher;

use rand_pcg::Mcg128Xsl64;

use mancala_rust::{Board, DepthSearchAI, MCTree, ScoreDiffEvaluator, AI};

#[bench]
fn mctree_with_stealing(b: &mut Bencher) {
    let mut ai = MCTree::new(10, Mcg128Xsl64::new(1));
    b.iter(|| ai.sow(&Board::new(true)));
}

#[bench]
fn mctree_no_stealing(b: &mut Bencher) {
    let mut ai = MCTree::new(10, Mcg128Xsl64::new(1));
    b.iter(|| ai.sow(&Board::new(false)));
}

#[bench]
fn dfs5_with_stealing(b: &mut Bencher) {
    let mut ai = DepthSearchAI::new(ScoreDiffEvaluator::new(), 5);
    b.iter(|| ai.sow(&Board::new(true)));
}

#[bench]
fn dfs5_no_stealing(b: &mut Bencher) {
    let mut ai = DepthSearchAI::new(ScoreDiffEvaluator::new(), 5);
    b.iter(|| ai.sow(&Board::new(false)));
}

#[bench]
fn dfs6_with_stealing(b: &mut Bencher) {
    let mut ai = DepthSearchAI::new(ScoreDiffEvaluator::new(), 6);
    b.iter(|| ai.sow(&Board::new(true)));
}

#[bench]
fn dfs6_no_stealing(b: &mut Bencher) {
    let mut ai = DepthSearchAI::new(ScoreDiffEvaluator::new(), 6);
    b.iter(|| ai.sow(&Board::new(false)));
}
