//! ユーザーが関与する・しないを問わず, ゲーム中の設定に関するリソースを定義する.

use bevy::prelude::*;

/// いわゆるハイスピ. BASE_SPEED定数があるので倍率で指定.
#[derive(Resource, Deref, DerefMut)]
pub struct NoteSpeed(pub f32);

impl Default for NoteSpeed {
    fn default() -> Self {
        NoteSpeed(1.0)
    }
}

/// bpmを表すリソース
#[derive(Resource, Deref, DerefMut)]
pub struct Bpm(pub f32);

/// 拍子を表すリソース
#[derive(Resource, Deref, DerefMut)]
pub struct Beat(pub u32);

#[derive(Clone, Copy, Resource, Debug)]
pub enum GameDifficulty {
    Normal,
    // パターン取得が解禁
    Expert,
    // ハードではアドリブノーツが取得可能
    Master,
}
impl GameDifficulty {
    pub fn get_color(&self) -> Color {
        match *self {
            GameDifficulty::Normal => Color::GREEN,
            GameDifficulty::Expert => Color::YELLOW,
            GameDifficulty::Master => Color::BLACK,
        }
    }
}
impl std::fmt::Display for GameDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
