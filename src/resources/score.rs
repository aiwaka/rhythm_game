use bevy::{prelude::*, utils::HashMap};

use crate::{components::receptor::NotesPattern, constants::MISS_THR};

/// Perfect以外は遅いか早いかをもたせる
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CatchEval {
    Perfect,
    NearPerfect(TimingEval),
    Ok(TimingEval),
    Miss,
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
            CatchEval::Miss => {
                write!(f, "miss")
            }
        }
    }
}
impl CatchEval {
    pub fn new(target_time: f64, real_time: f64) -> Self {
        match real_time - target_time {
            // この分岐は起こらないはず
            diff if diff < -MISS_THR => Self::Miss,
            diff if (-MISS_THR..=-MISS_THR / 3.0).contains(&diff) => Self::Ok(TimingEval::Fast),
            diff if (-MISS_THR / 3.0..=-MISS_THR / 6.0).contains(&diff) => {
                Self::NearPerfect(TimingEval::Fast)
            }
            diff if (MISS_THR / 6.0..=MISS_THR / 3.0).contains(&diff) => {
                Self::NearPerfect(TimingEval::Slow)
            }
            diff if (MISS_THR / 3.0..=MISS_THR).contains(&diff) => Self::Ok(TimingEval::Slow),
            diff if diff > MISS_THR => Self::Miss,
            _ => Self::Perfect,
        }
    }

    pub fn get_score(&self) -> u32 {
        match self {
            CatchEval::Perfect => 2,
            CatchEval::NearPerfect(_) => 2,
            CatchEval::Ok(_) => 1,
            CatchEval::Miss => 0,
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            CatchEval::Perfect => Color::GOLD,
            CatchEval::NearPerfect(_) => Color::GOLD,
            CatchEval::Ok(_) => Color::GREEN,
            CatchEval::Miss => Color::GRAY,
        }
    }
    pub fn get_timing(&self) -> Option<TimingEval> {
        match self {
            CatchEval::Perfect => None,
            CatchEval::NearPerfect(timing) => Some(timing),
            CatchEval::Ok(timing) => Some(timing),
            CatchEval::Miss => None,
        }
        .cloned()
    }
}

#[derive(Default, Debug, Resource)]
pub struct ScoreResource {
    score: usize,

    pattern_vec: Vec<NotesPattern>,
    eval_storage: HashMap<CatchEval, u32>,
}
impl ScoreResource {
    /// 取得数を増やし, スコアを増加させる.
    pub fn update_score(&mut self, catch_eval: &CatchEval) {
        if let Some(prev_val) = self.eval_storage.get_mut(catch_eval) {
            *prev_val += 1;
        } else {
            self.eval_storage.insert(*catch_eval, 1);
        }

        self.score += catch_eval.get_score() as usize;
    }

    pub fn add_score(&mut self, score: u32) {
        self.score += score as usize;
    }

    pub fn score(&self) -> usize {
        self.score
    }
    pub fn get_eval_num(&self, key: &CatchEval) -> u32 {
        if let Some(res) = self.eval_storage.get(key) {
            *res
        } else {
            0
        }
    }

    pub fn push_pattern(&mut self, pattern: NotesPattern) {
        self.pattern_vec.push(pattern);
        self.add_score(pattern.to_score());
    }
}
