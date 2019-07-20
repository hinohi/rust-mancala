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
        0.0134, 0.2610, 0.3636, 0.4477, 0.5152, 0.5803, 0.6911, 0.3212, 0.2324, 0.1876, 0.1828,
        0.1743, 0.2008, 0.3733, 0.2392, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.0348, 0.2760, 0.3517, 0.4232, 0.4769, 0.5833, 0.2349, 0.1664, 0.1432, 0.1324, 0.1258,
        0.1433, 0.3575, 0.4819, 0.3892, 0.4603, 0.5291, 0.3504, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.0950, 0.2782, 0.3444, 0.3932, 0.4914, 0.1392, 0.0795, 0.0420, 0.0142, 0.0155, 0.0609,
        0.2893, 0.3998, 0.5545, 0.4434, 0.4831, 0.3292, 0.5693, 0.1517, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.1622, 0.2935, 0.3457, 0.4295, 0.0812, 0.0257, -0.0061, -0.0211, -0.0095, 0.0488, 0.2528,
        0.3193, 0.4764, 0.6186, 0.4699, 0.4140, 0.6557, 0.3363, 0.2752, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.2474, 0.3202, 0.3830, 0.0553, 0.0130, -0.0075, -0.0065, 0.0223, 0.1018, 0.2658, 0.3179,
        0.4428, 0.5609, 0.6758, 0.4242, 0.6212, 0.4210, 0.3852, 0.3433, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.2549, 0.3326, 0.1126, 0.1079, 0.1269, 0.1693, 0.2373, 0.3506, 0.4695, 0.5100, 0.6226,
        0.7172, 0.7984, 0.8891, 0.9289, 0.8018, 0.7871, 0.7322, 0.6753, 0.6433, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
];

static OPPOSITE_SCORE_MAP: [[f64; 32]; 6] = [
    [
        0.3564, 0.1901, 0.1054, 0.0538, 0.0215, -0.0434, 0.0800, 0.2292, 0.3095, 0.3551, 0.3829,
        0.3575, 0.2987, 0.1155, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.3622, 0.1778, 0.1109, 0.0651, 0.0181, 0.0268, 0.2280, 0.2968, 0.3212, 0.3352, 0.3486,
        0.3565, 0.1713, 0.0882, 0.1566, 0.0590, -0.0826, -0.1569, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.3373, 0.1762, 0.1174, 0.0718, 0.0336, 0.2714, 0.3297, 0.3591, 0.3906, 0.4078, 0.3969,
        0.1945, 0.1228, 0.0259, 0.1152, 0.0582, 0.1613, -0.1205, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.2964, 0.1646, 0.1145, 0.0693, 0.3126, 0.3566, 0.3891, 0.4075, 0.4094, 0.3727, 0.1859,
        0.1574, 0.0498, -0.0576, 0.0628, 0.0842, -0.1596, 0.0312, 0.0208, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.2291, 0.1347, 0.1141, 0.3409, 0.3860, 0.4051, 0.4089, 0.3910, 0.3139, 0.1413, 0.1212,
        0.0394, -0.0559, -0.1635, 0.0111, -0.1528, -0.0434, -0.0705, -0.1453, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.2368, 0.1579, 0.2982, 0.2986, 0.2806, 0.2439, 0.1820, 0.0565, -0.1036, -0.1500, -0.2577,
        -0.3661, -0.4592, -0.5659, -0.6150, -0.5186, -0.5183, -0.4894, -0.4772, -0.4496, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
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
