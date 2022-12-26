use bevy::prelude::{Component, Resource};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SongDataParser {
    pub name: String,
    /// 画像名で指定
    pub thumbnail: String,
    pub config_file_name: String,
    /// エディットモードで扱えるかどうかのフラグ. 立てなくても良いようにOption付き.
    /// Noneはtrueとして扱い, trueなら編集不可とする. 編集可能にする場合falseにする.
    pub edit_freeze: Option<bool>,
}

#[derive(Resource, Component, Debug, Clone)]
pub struct SongData {
    pub name: String,
    pub thumbnail: String,
    pub config_file_name: String,
    pub edit_freeze: bool,
}
impl From<SongDataParser> for SongData {
    fn from(data: SongDataParser) -> Self {
        Self {
            name: data.name,
            thumbnail: data.thumbnail,
            config_file_name: data.config_file_name,
            edit_freeze: data.edit_freeze.unwrap_or(true),
        }
    }
}

/// 全曲データをロードして選曲ステートに受け渡すためのリソース.
#[derive(Resource)]
pub struct AllSongData(pub Vec<SongData>);
