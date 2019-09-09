use ndarray::Array1;
use rand::Rng;
use rust_nn::predict::{NN4Regression, NN6Regression, Regression};

use super::utils::{random_down, random_down_with_weight};
use super::{Evaluator, Score};
use crate::board::Board;

impl Score for i32 {
    const MIN: Self = std::i32::MIN + 1;
    const MAX: Self = std::i32::MAX - 1;
    #[inline]
    fn flip(&self) -> Self {
        -*self
    }
}

impl Score for i8 {
    const MIN: Self = std::i8::MIN + 1;
    const MAX: Self = std::i8::MAX - 1;
    #[inline]
    fn flip(&self) -> Self {
        -*self
    }
}

impl Score for f64 {
    const MIN: Self = -1e308;
    const MAX: Self = 1e308;
    #[inline]
    fn flip(&self) -> Self {
        -*self
    }
}

impl Score for f32 {
    const MIN: Self = -1e38;
    const MAX: Self = 1e38;
    #[inline]
    fn flip(&self) -> Self {
        -*self
    }
}

// -- ScoreDiff

#[derive(Debug, Default)]
pub struct ScoreDiffEvaluator;

impl ScoreDiffEvaluator {
    pub fn new() -> ScoreDiffEvaluator {
        ScoreDiffEvaluator {}
    }
}

impl Evaluator for ScoreDiffEvaluator {
    type Score = i32;
    fn eval(&mut self, board: &Board) -> Self::Score {
        i32::from(board.last_score())
    }
}

// -- ScorePos

lazy_static! {
    static ref POS1_SCORE_MAP: [[f64; 32]; 12] = {
        let mut score = [[0.0; 32]; 12];
        let data = include_str!("pos1_score.txt");
        let mut words = data.lines();
        for col in &mut score {
            for s in col {
                *s = words.next().unwrap().parse().unwrap();
            }
        }
        score
    };
}

#[derive(Debug, Default)]
pub struct ScorePosEvaluator;

impl ScorePosEvaluator {
    pub fn new() -> ScorePosEvaluator {
        ScorePosEvaluator {}
    }
}

impl Evaluator for ScorePosEvaluator {
    type Score = f64;
    fn eval(&mut self, board: &Board) -> Self::Score {
        if board.is_finished() {
            f64::from(board.last_score())
        } else {
            let mut score = 0.0;
            for (pos, seed) in board.self_seeds().iter().enumerate() {
                score += POS1_SCORE_MAP[pos][*seed as usize];
            }
            for (pos, seed) in board.opposite_seed().iter().enumerate() {
                score += POS1_SCORE_MAP[pos + 6][*seed as usize]
            }
            score + f64::from(board.score())
        }
    }
}

// -- WinRate

#[derive(Debug, Copy, Clone, Default)]
pub struct WinRateScore {
    win: u32,
    draw: u32,
    lose: u32,
    score: i32,
}

impl WinRateScore {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn count(&mut self, score: i32) {
        if score > 0 {
            self.win += 1;
        } else if score == 0 {
            self.draw += 1;
        } else {
            self.lose += 1;
        }
        self.score += score;
    }

    pub fn rate(&self) -> (f64, f64, f64) {
        let n = f64::from(self.win + self.draw + self.lose);
        (
            f64::from(self.win) / n,
            f64::from(self.draw) / n,
            f64::from(self.score) / n,
        )
    }
}

impl PartialEq for WinRateScore {
    fn eq(&self, other: &Self) -> bool {
        self.win == other.win && self.draw == other.draw && self.score == other.score
    }
}

impl Eq for WinRateScore {}

impl Ord for WinRateScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.win, self.draw, self.score).cmp(&(other.win, other.draw, other.score))
    }
}

impl PartialOrd for WinRateScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::ops::Add for WinRateScore {
    type Output = WinRateScore;
    fn add(self, other: Self) -> Self::Output {
        WinRateScore {
            win: self.win + other.win,
            draw: self.draw + other.draw,
            lose: self.lose + other.lose,
            score: self.score + other.score,
        }
    }
}

impl Score for WinRateScore {
    const MIN: Self = WinRateScore {
        win: 0,
        draw: 0,
        lose: 0,
        score: std::i32::MIN + 1,
    };
    const MAX: Self = WinRateScore {
        win: std::u32::MAX,
        draw: std::u32::MAX,
        lose: 0,
        score: std::i32::MAX - 1,
    };
    fn flip(&self) -> Self {
        WinRateScore {
            win: self.lose,
            draw: self.draw,
            lose: self.win,
            score: -self.score,
        }
    }
}

#[derive(Debug)]
pub struct MCTreeEvaluator<R> {
    random: R,
    num: usize,
}

impl<R> MCTreeEvaluator<R> {
    pub fn new(random: R, num: usize) -> Self {
        MCTreeEvaluator { random, num }
    }

    pub fn set_num(&mut self, num: usize) {
        self.num = num;
    }
}

impl<R: Rng> Evaluator for MCTreeEvaluator<R> {
    type Score = WinRateScore;
    fn eval(&mut self, board: &Board) -> Self::Score {
        let mut score = Self::Score::default();
        for _ in 0..self.num {
            let s = random_down(&mut self.random, board.clone()).last_score();
            score.count(i32::from(s));
        }
        score
    }
}

#[derive(Debug)]
pub struct WeightedMCTreeEvaluator<R, E> {
    random: R,
    eval: E,
    num: usize,
}

impl<R, E> WeightedMCTreeEvaluator<R, E> {
    pub fn new(random: R, eval: E, num: usize) -> Self {
        WeightedMCTreeEvaluator { random, eval, num }
    }

    pub fn set_num(&mut self, num: usize) {
        self.num = num;
    }
}

impl<R, E> Evaluator for WeightedMCTreeEvaluator<R, E>
where
    R: Rng,
    E: Evaluator,
    E::Score: Into<f64>,
{
    type Score = WinRateScore;
    fn eval(&mut self, board: &Board) -> Self::Score {
        let mut score = Self::Score::default();
        for _ in 0..self.num {
            let s = random_down_with_weight(&mut self.random, &mut self.eval, board.clone())
                .last_score();
            score.count(i32::from(s));
        }
        score
    }
}

// -- Neural Network base

static NN4_TRUE_MODEL: &[u8] = include_bytes!("NN4_true.model");
static NN4_FALSE_MODEL: &[u8] = include_bytes!("NN4_false.model");
static NN6_TRUE_MODEL: &[u8] = include_bytes!("NN6_true.model");
static NN6_FALSE_MODEL: &[u8] = include_bytes!("NN6_false.model");

#[inline]
fn nn_eval<R: Regression>(
    nn: &mut R,
    input: &mut Array1<rust_nn::Float>,
    board: &Board,
) -> rust_nn::Float {
    use rust_nn::Float;

    if board.is_finished() {
        Float::from(board.last_score())
    } else {
        for (pos, &s) in board.self_seeds().iter().enumerate() {
            input[pos] = Float::from(s);
        }
        for (pos, &s) in board.opposite_seed().iter().enumerate() {
            input[pos + 6] = Float::from(s);
        }
        nn.predict(input) + Float::from(board.score())
    }
}

#[derive(Debug)]
pub struct NN4Evaluator {
    nn: NN4Regression,
    input: Array1<rust_nn::Float>,
}

impl NN4Evaluator {
    pub fn new(stealing: bool) -> NN4Evaluator {
        let mut model = if stealing {
            NN4_TRUE_MODEL
        } else {
            NN4_FALSE_MODEL
        };
        NN4Evaluator {
            nn: NN4Regression::new(&mut model),
            input: Array1::zeros(12),
        }
    }
}

impl Evaluator for NN4Evaluator {
    type Score = rust_nn::Float;
    fn eval(&mut self, board: &Board) -> Self::Score {
        nn_eval(&mut self.nn, &mut self.input, board)
    }
}

#[derive(Debug)]
pub struct NN6Evaluator {
    nn: NN6Regression,
    input: Array1<rust_nn::Float>,
}

impl NN6Evaluator {
    pub fn new(stealing: bool) -> NN6Evaluator {
        let mut model = if stealing {
            NN6_TRUE_MODEL
        } else {
            NN6_FALSE_MODEL
        };
        NN6Evaluator {
            nn: NN6Regression::new(&mut model),
            input: Array1::zeros(12),
        }
    }
}

impl Evaluator for NN6Evaluator {
    type Score = rust_nn::Float;
    fn eval(&mut self, board: &Board) -> Self::Score {
        nn_eval(&mut self.nn, &mut self.input, board)
    }
}
