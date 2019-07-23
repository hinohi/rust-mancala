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

lazy_static! {
    static ref POS1_SCORE_MAP: [[f64; 32]; 12] = {
        let mut score = [[0.0; 32]; 12];
        let data = include_str!("pos1_score.txt");
        let mut words = data.lines();
        for p in 0..12 {
            for s in 0..32 {
                score[p][s] = words.next().unwrap().parse().unwrap();
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
            let (s0, s1) = board.last_scores();
            if board.side() == Side::First {
                f64::from(s0) - f64::from(s1)
            } else {
                f64::from(s1) - f64::from(s0)
            }
        } else {
            let mut score = 0.0;
            for (pos, seed) in board.self_seeds().iter().enumerate() {
                score += POS1_SCORE_MAP[pos][*seed as usize];
            }
            for (pos, seed) in board.opposite_seed().iter().enumerate() {
                score += POS1_SCORE_MAP[pos + 6][*seed as usize]
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

lazy_static! {
    static ref POS2_SCORE_MAP: [[[[f64; 32]; 12]; 32]; 12] = {
        let mut score = [[[[0.0; 32]; 12]; 32]; 12];
        let data = include_str!("pos2_score.txt");
        let mut words = data.lines();
        for p1 in 0..12 {
            for s1 in 0..32 {
                for p2 in 0..12 {
                    for s2 in 0..32 {
                        score[p1][s1][p2][s2] = words.next().unwrap().parse().unwrap();
                    }
                }
            }
        }
        score
    };
}

#[derive(Debug, Default)]
pub struct ScorePos2Evaluator;

impl ScorePos2Evaluator {
    pub fn new() -> ScorePos2Evaluator {
        ScorePos2Evaluator {}
    }
}

impl Evaluator for ScorePos2Evaluator {
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
            let seeds = {
                let mut seeds = [0; 12];
                for (pos, seed) in board.self_seeds().iter().enumerate() {
                    seeds[pos] = *seed as usize;
                }
                for (pos, seed) in board.opposite_seed().iter().enumerate() {
                    seeds[pos + 6] = *seed as usize;
                }
                seeds
            };
            for (p1, s1) in seeds.iter().enumerate() {
                for (p2, s2) in seeds.iter().enumerate() {
                    score += POS2_SCORE_MAP[p1][*s1][p2][*s2];
                }
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
