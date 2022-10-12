use bevy::prelude::*;

use crate::resources::song_list::SongDataToml;

#[derive(Component)]
pub struct SongSelectText;

/// リストにくっつける, 現在選ばれている番号を示す
#[derive(Component)]
pub struct ActiveSongCard(pub usize);

/// 曲カード. カード番号を保存
#[derive(Component)]
pub struct SongSelectCard(pub usize);

#[derive(Component, Clone, Debug)]
pub struct SongData {
    pub name: String,
    pub thumbnail: i32,
    pub config_file_name: String,
}
impl SongData {
    pub fn new(toml_data: &SongDataToml) -> Self {
        Self {
            name: toml_data.name.clone(),
            thumbnail: toml_data.thumbnail,
            config_file_name: toml_data.config_file_name.clone(),
        }
    }
}
