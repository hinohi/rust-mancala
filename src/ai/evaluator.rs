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
        -0.0, 3.1, 4.4, 5.4, 6.2, 7.0, 8.4, 4.0, 3.0, 2.5, 2.6, 2.8, 3.6, 6.4, 6.7, 17.5, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.2, 3.3, 4.3, 5.1, 5.8, 7.1, 2.9, 2.1, 1.9, 1.8, 1.8, 2.1, 4.7, 6.3, 5.5, 6.7, 8.6, 8.8,
        14.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.9, 3.3, 4.2, 4.8, 5.9, 1.7, 1.0, 0.6, 0.3, 0.4, 1.0, 3.7, 5.1, 7.1, 6.1, 6.8, 5.6, 8.8,
        6.2, 4.7, 21.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.7, 3.5, 4.2, 5.1, 1.0, 0.4, 0.0, -0.1, 0.0, 0.8, 3.2, 4.1, 6.1, 7.9, 6.5, 6.1, 9.0, 5.9,
        6.5, 7.3, 14.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        2.8, 3.8, 4.4, 0.7, 0.2, -0.0, 0.0, 0.4, 1.4, 3.4, 4.0, 5.6, 7.0, 8.5, 5.9, 8.3, 6.3, 6.6,
        7.5, 9.2, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        3.0, 3.9, 1.3, 1.3, 1.5, 2.0, 2.8, 4.0, 5.6, 6.3, 7.6, 8.8, 9.8, 11.0, 11.5, 10.2, 10.3,
        10.0, 10.2, 11.1, 13.2, 13.8, 22.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
];

static OPPOSITE_SCORE_MAP: [[f64; 32]; 6] = [
    [
        4.2, 2.2, 1.2, 0.5, 0.1, -0.6, 0.7, 2.4, 3.2, 3.6, 3.6, 2.8, 1.2, -1.2, -1.8, -15.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        4.3, 2.1, 1.2, 0.7, 0.1, 0.2, 2.5, 3.3, 3.5, 3.5, 3.5, 3.3, 1.0, -0.1, 0.3, -1.3, -3.0,
        -4.3, -9.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        4.0, 2.0, 1.3, 0.8, 0.3, 3.1, 3.7, 4.0, 4.3, 4.4, 4.1, 1.6, 0.6, -0.7, -0.2, -1.0, -0.6,
        -3.4, -2.6, 2.9, -19.8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        3.6, 1.9, 1.3, 0.7, 3.6, 4.1, 4.4, 4.6, 4.5, 4.0, 1.7, 1.2, -0.2, -1.8, -0.9, -1.0, -3.6,
        -2.2, -3.3, -5.0, -13.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        2.8, 1.5, 1.3, 4.0, 4.5, 4.7, 4.6, 4.4, 3.4, 1.3, 0.9, -0.2, -1.4, -2.8, -1.4, -3.2, -2.6,
        -3.6, -5.5, -8.8, -21.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        2.9, 1.8, 3.5, 3.5, 3.2, 2.7, 1.9, 0.4, -1.6, -2.2, -3.6, -5.0, -6.2, -7.6, -8.3, -7.6,
        -8.1, -8.3, -9.2, -10.1, -14.1, -14.5, -21.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
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
