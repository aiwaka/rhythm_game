use bevy::prelude::*;

pub struct SpawnTimer(pub Timer);

/// 曲再生を開始するゲーム開始からの時間（秒）
pub struct AudioStartTime(pub f64);

/// いわゆるハイスピ. BASE_SPEED定数があるので倍率で指定.
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(1.0)
    }
}

pub struct Bpm(pub f32);
