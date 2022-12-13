use bevy::prelude::{Component, Resource};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SongDataParser {
    pub name: String,
    /// TODO: 現在はダミーデータ. サムネイル画像を参照できる形式に将来的に置き換える.
    pub thumbnail: i32,
    pub config_file_name: String,
}

#[derive(Resource, Component, Debug, Clone)]
pub struct SongData {
    pub name: String,
    pub thumbnail: i32,
    pub config_file_name: String,
}
impl From<SongDataParser> for SongData {
    fn from(data: SongDataParser) -> Self {
        Self {
            name: data.name,
            thumbnail: data.thumbnail,
            config_file_name: data.config_file_name,
        }
    }
}

/// 全曲データをロードして選曲ステートに受け渡すためのリソース.
#[derive(Resource)]
pub struct AllSongData(pub Vec<SongData>);
