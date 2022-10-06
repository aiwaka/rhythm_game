use bevy::prelude::*;

use crate::game_constants::{DISTANCE, NOTE_BASE_SPEED};

use serde_derive::{Deserialize, Serialize};

#[derive(Component)]
pub struct Note {
    pub speed: Speed,
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
            0 => 150.0,
            1 => 50.0,
            2 => -50.0,
            3 => -150.0,
            _ => 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}
impl Speed {
    /// Returns actual speed at which the arrow should move
    pub fn value(&self) -> f32 {
        NOTE_BASE_SPEED * self.multiplier()
    }
    /// Speed multiplier
    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoteTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub key_column: KeyColumn,
}
impl NoteTime {
    pub fn new(arrow: &NoteTimeToml) -> Self {
        let speed_value = arrow.speed.value();
        Self {
            spawn_time: arrow.click_time - (DISTANCE / speed_value) as f64,
            speed: arrow.speed,
            key_column: arrow.key_column,
        }
    }
}

#[derive(Debug)]
pub struct SongConfig {
    pub name: String,
    pub notes: Vec<NoteTime>,
}

/// use for toml
#[derive(Deserialize, Debug)]
pub struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub notes: Vec<NoteTimeToml>,
}

/// use for toml
#[derive(Deserialize, Serialize, Debug)]
pub struct NoteTimeToml {
    pub click_time: f64,
    pub speed: Speed,
    pub key_column: KeyColumn,
}
