use bevy::prelude::*;

pub struct SpawnTimer(pub Timer);

/// いわゆるハイスピ. BASE_SPEED定数があるので倍率で指定.
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(1.0)
    }
}
