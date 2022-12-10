use bevy::prelude::Resource;
use serde_derive::Deserialize;

use crate::components::song_select::SongData;

/// use for toml
#[derive(Deserialize, Debug)]
pub struct SongDataToml {
    pub name: String,
    /// TODO: 現在はダミーデータ. サムネイル画像を参照できる形式に将来的に置き換える.
    pub thumbnail: i32,
    pub config_file_name: String,
}

/// 全曲データをロードして選曲ステートに受け渡すためのリソース.
#[derive(Resource)]
pub struct AllSongData(pub Vec<SongData>);
