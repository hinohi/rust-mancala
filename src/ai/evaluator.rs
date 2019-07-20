use rand::Rng;

use super::{utils::random_down, Evaluator, Score};
use crate::board::{Board, Side};

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
    const MIN: Self = -1267650600228229401496703205376.0;
    const MAX: Self = 1267650600228229401496703205376.0;
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
        let (s0, s1) = board.last_scores();
        if board.side() == Side::First {
            i32::from(s0) - i32::from(s1)
        } else {
            i32::from(s1) - i32::from(s0)
        }
    }
}

// -- ScorePos

static SELF_SCORE_MAP: [[f64; 32]; 6] = [
    [
        0.16, 3.13, 4.36, 5.37, 6.18, 6.96, 8.29, 3.85, 2.79, 2.25, 2.19, 2.09, 2.41, 4.48, 2.87,
        16.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.42, 3.31, 4.22, 5.08, 5.72, 7.00, 2.82, 2.00, 1.72, 1.59, 1.51, 1.72, 4.29, 5.78, 4.67,
        5.52, 6.35, 4.20, 9.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.14, 3.34, 4.13, 4.72, 5.90, 1.67, 0.95, 0.50, 0.17, 0.19, 0.73, 3.47, 4.80, 6.65, 5.32,
        5.80, 3.95, 6.83, 1.82, 1.44, 19.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.95, 3.52, 4.15, 5.15, 0.97, 0.31, -0.07, -0.25, -0.11, 0.59, 3.03, 3.83, 5.72, 7.42,
        5.64, 4.97, 7.87, 4.04, 3.30, 2.27, 9.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0,
    ],
    [
        2.97, 3.84, 4.60, 0.66, 0.16, -0.09, -0.08, 0.27, 1.22, 3.19, 3.81, 5.31, 6.73, 8.11, 5.09,
        7.45, 5.05, 4.62, 4.12, 2.91, -4.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        3.06, 3.99, 1.35, 1.30, 1.52, 2.03, 2.85, 4.21, 5.63, 6.12, 7.47, 8.61, 9.58, 10.67, 11.15,
        9.62, 9.44, 8.79, 8.10, 7.72, 8.77, 8.50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
];

static OPPOSITE_SCORE_MAP: [[f64; 32]; 6] = [
    [
        4.28, 2.28, 1.26, 0.65, 0.26, -0.52, 0.96, 2.75, 3.71, 4.26, 4.59, 4.29, 3.58, 1.39, 1.46,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        4.35, 2.13, 1.33, 0.78, 0.22, 0.32, 2.74, 3.56, 3.85, 4.02, 4.18, 4.28, 2.06, 1.06, 1.88,
        0.71, -0.99, -1.88, -10.57, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0,
    ],
    [
        4.05, 2.11, 1.41, 0.86, 0.40, 3.26, 3.96, 4.31, 4.69, 4.89, 4.76, 2.33, 1.47, 0.31, 1.38,
        0.70, 1.94, -1.45, 0.74, 6.38, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        3.56, 1.97, 1.37, 0.83, 3.75, 4.28, 4.67, 4.89, 4.91, 4.47, 2.23, 1.89, 0.60, -0.69, 0.75,
        1.01, -1.91, 0.37, 0.25, -1.95, 1.50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0,
    ],
    [
        2.75, 1.62, 1.37, 4.09, 4.63, 4.86, 4.91, 4.69, 3.77, 1.70, 1.45, 0.47, -0.67, -1.96, 0.13,
        -1.83, -0.52, -0.85, -1.74, -3.12, -22.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0,
    ],
    [
        2.84, 1.90, 3.58, 3.58, 3.37, 2.93, 2.18, 0.68, -1.24, -1.80, -3.09, -4.39, -5.51, -6.79,
        -7.38, -6.22, -6.22, -5.87, -5.73, -5.40, -8.33, -8.71, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
    ],
];

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
            let (s0, s1) = board.last_scores();
            if board.side() == Side::First {
                f64::from(s0) - f64::from(s1)
            } else {
                f64::from(s1) - f64::from(s0)
            }
        } else {
            let mut score = 0.0;
            for (pos, seed) in board.self_seeds().iter().enumerate() {
                score += SELF_SCORE_MAP[pos][*seed as usize];
            }
            for (pos, seed) in board.opposite_seed().iter().enumerate() {
                score += OPPOSITE_SCORE_MAP[pos][*seed as usize]
            }
            let (s0, s1) = board.scores();
            if board.side() == Side::First {
                score + f64::from(s0) - f64::from(s1)
            } else {
                score + f64::from(s1) - f64::from(s0)
            }
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
            let (s0, s1) = random_down(&mut self.random, board.clone()).last_scores();
            score.count(if board.side() == Side::First {
                i32::from(s0) - i32::from(s1)
            } else {
                i32::from(s1) - i32::from(s0)
            });
        }
        score
    }
}
