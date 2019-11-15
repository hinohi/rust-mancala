use crate::ai::Score;

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
