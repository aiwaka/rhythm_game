use std::collections::VecDeque;

use serde_derive::{Deserialize, Serialize};

use crate::components::{note::Note, song_select::SongData};

/// 選択された曲をロードする際に知るためのリソース.
/// SongDataと同じフィールドを持つが, 名前で使い方を決めていると考える
pub struct SelectedSong {
    pub name: String,
    pub filename: String,
}
impl SelectedSong {
    pub fn from_song_card(data: &SongData) -> Self {
        Self {
            name: data.name.clone(),
            filename: data.config_file_name.clone(),
        }
    }
}

/// 曲再生を開始するゲーム開始からの時間（秒）
pub struct AudioStartTime(pub f64);

/// いわゆるハイスピ. BASE_SPEED定数があるので倍率で指定.
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(1.0)
    }
}

#[derive(Debug)]
pub struct SongConfig {
    pub name: String,
    pub music_filename: String,
    pub bpm: f32,
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
    pub bpm: f32,
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
