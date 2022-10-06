use crate::game_constants::THRESHOLD;

#[derive(Default, Debug)]
pub struct ScoreResource {
    corrects: usize,
    fails: usize,

    score: usize,
}
impl ScoreResource {
    /// 取得数を増やし, スコアを増加させ, スコアの増分を返す
    pub fn increase_correct(&mut self, distance: f32) -> usize {
        self.corrects += 1;

        // ボタン押下の近さに応じて[0, 1]の値をとる.
        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;
        // [10, 100]点を与える
        let points = (score_multiplier * 100.0).clamp(10.0, 100.0) as usize;
        self.score += points;

        points
    }

    pub fn increase_fails(&mut self) {
        self.fails += 1;
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
}
