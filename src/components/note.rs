use bevy::prelude::*;

use crate::{
    game_constants::{DISTANCE, NOTE_BASE_SPEED},
    resources::song::NoteTimeToml,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Component)]
pub struct Note {
    pub key_column: KeyColumn,
}

/// 鍵盤番号
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct KeyColumn(pub i32);

impl KeyColumn {
    /// 番号とキーを結びつけ押されたかどうかを取得
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self.0 {
            0 => [KeyCode::C],
            1 => [KeyCode::V],
            2 => [KeyCode::N],
            3 => [KeyCode::M],
            _ => [KeyCode::Return],
        };
        input.any_just_pressed(keys)
    }

    /// 鍵盤番号に対応させたx座標を計算
    pub fn x_coord(&self) -> f32 {
        match self.0 {
            0 => -150.0,
            1 => -50.0,
            2 => 50.0,
            3 => 150.0,
            _ => 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoteTime {
    /// 出現時間. 決められた拍に判定線に来るように設定される.
    pub spawn_time: f64,
    pub key_column: KeyColumn,
}
impl NoteTime {
    pub fn new(note: &NoteTimeToml, beat_par_bar: u32, bpm: f64, speed_coeff: f32) -> Self {
        // 座標の移動速度. BASE_SPEED * 倍率.
        let speed = speed_coeff * NOTE_BASE_SPEED;
        let second_par_beat = bpm.recip() * 60.0;
        // 判定線に到達する時間を曲開始時刻から測ったもの.
        let click_time = (beat_par_bar * note.bar + note.beat) as f64 * second_par_beat;
        Self {
            spawn_time: click_time - ((DISTANCE / speed) as f64).abs(),
            key_column: note.key_column,
        }
    }
}
