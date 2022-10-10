use bevy::prelude::*;

use crate::{components::receptor::NotesPattern, game_constants::ERROR_THRESHOLD};

/// Perfect以外は遅いか早いかをもたせる
#[derive(Debug, Clone, Copy)]
pub enum TimingEval {
    Slow,
    Fast,
}
impl std::fmt::Display for TimingEval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimingEval::Slow => {
                write!(f, "slow")
            }
            TimingEval::Fast => {
                write!(f, "fast")
            }
        }
    }
}
impl TimingEval {
    pub fn get_color(&self) -> Color {
        match self {
            TimingEval::Slow => Color::BLUE,
            TimingEval::Fast => Color::ORANGE,
        }
    }
}

/// ノーツ取得の評価
#[derive(Debug, Clone, Copy)]
pub enum CatchEval {
    Perfect,
    NearPerfect(TimingEval),
    Ok(TimingEval),
    Miss(TimingEval),
}
impl std::fmt::Display for CatchEval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatchEval::Perfect => {
                write!(f, "perfect")
            }
            CatchEval::NearPerfect(_) => {
                write!(f, "perfect")
            }
            CatchEval::Ok(_) => {
                write!(f, "ok")
            }
            CatchEval::Miss(_) => {
                write!(f, "miss")
            }
        }
    }
}
impl CatchEval {
    pub fn new(target_time: f64, real_time: f64) -> Self {
        match real_time - target_time {
            diff if diff < -ERROR_THRESHOLD => Self::Miss(TimingEval::Fast),
            diff if (-ERROR_THRESHOLD..=-ERROR_THRESHOLD / 3.0).contains(&diff) => {
                Self::Ok(TimingEval::Fast)
            }
            diff if (-ERROR_THRESHOLD / 3.0..=-ERROR_THRESHOLD / 6.0).contains(&diff) => {
                Self::NearPerfect(TimingEval::Fast)
            }
            diff if (ERROR_THRESHOLD / 6.0..=ERROR_THRESHOLD / 3.0).contains(&diff) => {
                Self::NearPerfect(TimingEval::Slow)
            }
            diff if (ERROR_THRESHOLD / 3.0..=ERROR_THRESHOLD).contains(&diff) => {
                Self::Ok(TimingEval::Slow)
            }
            diff if diff > ERROR_THRESHOLD => Self::Miss(TimingEval::Slow),
            _ => Self::Perfect,
        }
    }

    pub fn get_score(&self) -> u32 {
        match self {
            CatchEval::Perfect => 2,
            CatchEval::NearPerfect(_) => 2,
            CatchEval::Ok(_) => 1,
            CatchEval::Miss(_) => 0,
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            CatchEval::Perfect => Color::GOLD,
            CatchEval::NearPerfect(_) => Color::GOLD,
            CatchEval::Ok(_) => Color::GREEN,
            CatchEval::Miss(_) => Color::GRAY,
        }
    }
    pub fn get_timing(&self) -> Option<TimingEval> {
        match self {
            CatchEval::Perfect => None,
            CatchEval::NearPerfect(timing) => Some(timing),
            CatchEval::Ok(timing) => Some(timing),
            CatchEval::Miss(timing) => Some(timing),
        }
        .cloned()
    }
}

#[derive(Default, Debug)]
pub struct ScoreResource {
    corrects: usize,
    fails: usize,

    score: usize,

    pattern_vec: Vec<NotesPattern>,
}
impl ScoreResource {
    /// 取得数を増やし, スコアを増加させ, スコアの増分を返す.
    pub fn increase_correct(&mut self, catch_eval: &CatchEval) {
        self.corrects += 1;

        self.score += catch_eval.get_score() as usize;
    }

    pub fn increase_fails(&mut self) {
        self.fails += 1;
    }

    pub fn add_score(&mut self, score: u32) {
        self.score += score as usize;
    }

    pub fn score(&self) -> usize {
        self.score
    }
    pub fn corrects(&self) -> usize {
        self.corrects
    }
    pub fn fails(&self) -> usize {
        self.fails
    }

    pub fn push_pattern(&mut self, pattern: NotesPattern) {
        self.pattern_vec.push(pattern);
        self.add_score(pattern.to_score());
    }
}
