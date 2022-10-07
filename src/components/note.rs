use bevy::prelude::*;

use crate::game_constants::{DISTANCE, NOTE_BASE_SPEED};

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
    pub spawn_time: f64,
    pub key_column: KeyColumn,
}
impl NoteTime {
    pub fn new(note: &NoteTimeToml, speed_coeff: f32) -> Self {
        // 座標の移動速度. BASE_SPEED * 倍率.
        let speed = speed_coeff * NOTE_BASE_SPEED;
        Self {
            spawn_time: note.click_time - (DISTANCE / speed) as f64,
            key_column: note.key_column,
        }
    }
}

#[derive(Debug)]
pub struct SongConfig {
    pub name: String,
    pub music_filename: String,
    pub bpm: f32,
    pub notes: Vec<NoteTime>,
}

/// use for toml
#[derive(Deserialize, Debug)]
pub struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub bpm: f32,
    pub notes: Vec<NoteTimeToml>,
}

/// use for toml
#[derive(Deserialize, Serialize, Debug)]
pub struct NoteTimeToml {
    pub click_time: f64,
    pub key_column: KeyColumn,
}
