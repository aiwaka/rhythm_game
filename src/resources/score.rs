use crate::{components::receptor::NotesPattern, game_constants::ERROR_THRESHOLD};

/// Perfect以外は遅いか早いかをもたせる
#[derive(Debug)]
pub enum TimingEval {
    Slow,
    Fast,
}

/// ノーツ取得の評価
#[derive(Debug)]
pub enum CatchEval {
    Perfect,
    Ok(TimingEval),
    Miss(TimingEval),
}
impl CatchEval {
    pub fn new(target_time: f64, real_time: f64) -> Self {
        match real_time - target_time {
            diff if diff < -ERROR_THRESHOLD => Self::Miss(TimingEval::Slow),
            diff if (-ERROR_THRESHOLD..=-ERROR_THRESHOLD / 3.0).contains(&diff) => {
                Self::Ok(TimingEval::Slow)
            }
            diff if (ERROR_THRESHOLD / 3.0..=ERROR_THRESHOLD).contains(&diff) => {
                Self::Ok(TimingEval::Fast)
            }
            diff if diff > ERROR_THRESHOLD => Self::Miss(TimingEval::Fast),
            _ => Self::Perfect,
        }
    }

    pub fn to_score(&self) -> u32 {
        match self {
            CatchEval::Perfect => 2,
            CatchEval::Ok(_timing) => 1,
            CatchEval::Miss(_timing) => 0,
        }
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

        self.score += catch_eval.to_score() as usize;
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
