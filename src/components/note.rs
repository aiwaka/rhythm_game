use bevy::prelude::*;

use crate::{
    game_constants::{DISTANCE, LANE_WIDTH, NOTE_BASE_SPEED},
    resources::song::NoteTimeToml,
};

/// 鍵盤レーン
#[derive(Component, Clone, Copy, Debug)]
pub struct KeyLane(pub i32);
impl KeyLane {
    /// 鍵盤の数
    pub const KEY_NUM: u8 = 4;

    /// 番号とキーを結びつけ, 指定された鍵盤番号に対応するキーが今押されたかどうかを取得.
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self.0 {
            0 => [KeyCode::C, KeyCode::D],
            1 => [KeyCode::V, KeyCode::F],
            2 => [KeyCode::N, KeyCode::J],
            3 => [KeyCode::M, KeyCode::K],
            _ => [KeyCode::Return, KeyCode::Return],
        };
        input.any_just_pressed(keys)
    }

    /// x_coordをi32から取得
    pub fn x_coord_from_num(num: i32) -> f32 {
        let half_width = LANE_WIDTH / 2.0;
        0.0 - (Self::KEY_NUM - 1) as f32 * half_width + LANE_WIDTH * num as f32
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Note {
    /// 出現時間. 決められた拍に判定線に来るように設定される.
    pub spawn_time: f64,
    pub target_time: f64,
    pub bar: u32,
    pub beat: f64,
    pub key_column: i32,
}
impl Note {
    pub fn new(note: &NoteTimeToml, beat_par_bar: u32, bpm: f32, speed_coeff: f32) -> Self {
        // 座標の移動速度. BASE_SPEED * 倍率.
        let speed = speed_coeff * NOTE_BASE_SPEED;
        let second_par_beat = bpm.recip() * 60.0;
        // 判定線に到達する時間を曲開始時刻から測ったもの.
        let click_time = ((beat_par_bar * note.bar) as f64 + note.beat) * second_par_beat as f64;
        Self {
            spawn_time: click_time - ((DISTANCE / speed) as f64).abs(),
            target_time: click_time,
            bar: note.bar,
            beat: note.beat,
            key_column: note.key_column,
        }
    }
}
