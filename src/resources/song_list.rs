use serde_derive::{Deserialize, Serialize};

/// use for toml
#[derive(Deserialize, Debug)]
pub struct SongDataToml {
    /// TODO: 現在はダミーデータ. サムネイル画像を参照できる形式に将来的に置き換える.
    pub thumbnail: i32,
    pub config_file_name: String,
}

#[derive(Debug)]
pub struct SongData {
    pub thumbnail: i32,
    pub config_file_name: String,
}
impl SongData {
    pub fn new(toml_data: &SongDataToml) -> Self {
        Self {
            thumbnail: toml_data.thumbnail,
            config_file_name: toml_data.config_file_name.clone(),
        }
    }
}
