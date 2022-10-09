use std::collections::VecDeque;

use serde_derive::{Deserialize, Serialize};

use crate::components::note::Note;

#[derive(Debug)]
pub struct SongConfig {
    pub name: String,
    pub music_filename: String,
    pub bpm: f64,
    /// 一小節あたりの拍数
    pub beat_par_bar: u32,
    pub notes: VecDeque<Note>,
}

/// use for toml
#[derive(Deserialize, Debug)]
pub struct SongConfigToml {
    pub name: String,
    pub filename: String,
    /// 一小節に何拍あるか
    pub beat_par_bar: u32,
    pub bpm: f64,
    pub notes: Vec<NoteTimeToml>,
}

/// TOMLファイルのノーツ情報パース用構造体
#[derive(Deserialize, Serialize, Debug)]
pub struct NoteTimeToml {
    /// 小節番号（0始まり）
    pub bar: u32,
    /// 小節内の拍位置（0始まり）. 1.5なら2拍目の裏になる
    pub beat: f64,
    /// 鍵盤の番号
    pub key_column: i32,
}
