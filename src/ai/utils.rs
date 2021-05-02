use rand::Rng;

use super::{Evaluator, Score};
use crate::board::Board;

struct Ordable<F>(pub F);

impl<F> PartialEq for Ordable<F>
where
    F: PartialEq,
{
    fn eq(&self, other: &Ordable<F>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<F> Eq for Ordable<F> where F: PartialEq {}

impl<F> PartialOrd for Ordable<F>
where
    F: PartialOrd,
{
    fn partial_cmp(&self, other: &Ordable<F>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<F> Ord for Ordable<F>
where
    F: PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn ab_search<E: Evaluator>(
    board: Board,
    eval: &mut E,
    depth: usize,
    alpha: E::Score,
    beta: E::Score,
) -> E::Score {
    if depth == 0 || board.is_finished() {
        return eval.eval(&board);
    }
    let mut list = board.list_next().drain().collect::<Vec<_>>();
    if depth >= 3 {
        list.sort_by_cached_key(|b| Ordable(eval.eval(b)));
    }
    let mut alpha = alpha;
    for next in list {
        let a = ab_search(next, eval, depth - 1, beta.flip(), alpha.flip()).flip();
        if a > alpha {
            alpha = a;
        }
        if alpha >= beta {
            break;
        }
    }
    alpha
}

pub fn random_down<R: Rng>(random: &mut R, board: &Board) -> Board {
    let mut board = board.clone();
    loop {
        let mut next_list = board.list_next().drain().collect::<Vec<_>>();
        if next_list.is_empty() {
            break;
        }
        let idx = random.gen_range(0..next_list.len());
        board = next_list.swap_remove(idx);
    }
    board
}

#[inline]
pub fn soft_max(x: &mut [f64]) {
    let max = x.iter().fold(f64::NAN, |x, v| x.max(*v));
    x.iter_mut()
        .map(|v| {
            *v = (*v - max).exp();
        })
        .last();
}

#[inline]
pub fn choice_with_weight<R: Rng>(random: &mut R, weight: &[f64]) -> usize {
    let sum = weight.iter().fold(0.0, |x, y| x + *y);
    let r = random.gen_range(0.0..sum);
    let mut p = 0.0;
    for (i, w) in weight.iter().enumerate() {
        p += *w;
        if r <= p {
            return i;
        }
    }
    weight.len() - 1
}

pub fn random_down_with_weight<R, E>(random: &mut R, eval: &mut E, board: Board) -> Board
where
    R: Rng,
    E: Evaluator,
    E::Score: Into<f64>,
{
    let mut board = board;
    loop {
        let mut next_list = board.list_next().drain().collect::<Vec<_>>();
        if next_list.is_empty() {
            break;
        }
        let prob = {
            let mut prob = next_list
                .iter()
                .map(|board| -eval.eval(board).into())
                .collect::<Vec<_>>();
            soft_max(&mut prob);
            prob
        };
        let idx = choice_with_weight(random, &prob);
        board = next_list.swap_remove(idx);
    }
    board
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{McTreeEvaluator, ScoreDiffEvaluator, WinRateScore};
    use rand_pcg::Mcg128Xsl64;

    #[test]
    fn smoke_ab_search_diff() {
        let mut eval = ScoreDiffEvaluator::new();
        ab_search(
            Board::new(true),
            &mut eval,
            5,
            <i32 as Score>::MIN,
            <i32 as Score>::MAX,
        );
    }

    #[test]
    fn smoke_ab_search_mc() {
        let mut eval = McTreeEvaluator::new(Mcg128Xsl64::new(1), 10);
        ab_search(
            Board::new(true),
            &mut eval,
            2,
            <WinRateScore as Score>::MIN,
            <WinRateScore as Score>::MAX,
        );
    }

    #[test]
    fn smoke_random_down() {
        let mut random = Mcg128Xsl64::new(1);
        let board = random_down(&mut random, &Board::new(false));
        assert!(board.is_finished());
    }

    #[test]
    fn test_soft_max() {
        let mut x = [-4.0, 10.0, 8.0, 0.0];
        soft_max(&mut x);
        for v in x.iter() {
            assert!(*v >= 0.0);
        }
        assert!(x[0] < x[1]);
        assert!(x[0] < x[2]);
        assert!(x[0] < x[3]);
        assert!(x[1] > x[2]);
        assert!(x[1] > x[3]);
        assert!(x[2] > x[3]);
    }

    #[test]
    fn test_choice_with_weight() {
        let x = [3.0, 0.0, 7.0];
        let mut random = Mcg128Xsl64::new(1);
        let mut count = [0; 3];
        for _ in 0..1000 {
            count[choice_with_weight(&mut random, &x)] += 1;
        }
        assert!(270 <= count[0] && count[0] <= 330);
        assert_eq!(count[1], 0);
        assert!(670 <= count[2] && count[2] <= 730);
    }

    #[test]
    fn smoke_random_down_with_weight() {
        let mut random = Mcg128Xsl64::new(1);
        let mut eval = ScoreDiffEvaluator::new();
        let board = random_down_with_weight(&mut random, &mut eval, Board::new(false));
        assert!(board.is_finished());
    }
}
