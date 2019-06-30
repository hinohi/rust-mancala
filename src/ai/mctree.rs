use rand::{seq::SliceRandom, Rng};

use super::base::AI;
use super::utils::ab_search;
use crate::game::{Board, Evaluation, ScoreDiffEvaluation, PIT};

#[derive(Debug)]
pub struct MCTree<R: Rng> {
    path_num: usize,
    search_start_num: u8,
    random: R,
}

impl<R> MCTree<R>
where
    R: Rng,
{
    pub fn new(path_num: usize, search_start_num: u8, random: R) -> MCTree<R> {
        MCTree {
            path_num,
            search_start_num,
            random,
        }
    }
}
