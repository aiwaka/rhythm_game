use bevy::{prelude::*, utils::HashMap};

use crate::resources::song_list::SongData;

use super::AssetHandles;

/// 曲セレクトシーンにおけるアセットハンドル
#[derive(Debug, Resource)]
pub struct SongSelectAssetHandles {
    // フォント
    pub main_font: Handle<Font>,

    // 画像
    pub background: Handle<Image>,

    /// サムネ画像メッシュ
    pub thumb_mesh: Handle<Mesh>,
    // サムネ用マテリアル
    pub thumb_img: HashMap<String, Handle<Image>>,
}

impl SongSelectAssetHandles {
    pub fn new(
        server: &Res<AssetServer>,
        _texture_atlas: &mut ResMut<Assets<TextureAtlas>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        song_data: &[SongData],
    ) -> Self {
        // let numbers = server.load("images/numbers.png");
        let thumb_shape = shape::Quad::new(Vec2::new(80.0, 80.0 * 1.6));
        let mut thumb_img = HashMap::<String, Handle<Image>>::new();
        for data in song_data {
            let img = server.load(format!("images/thumb/{}", data.thumbnail));
            thumb_img.insert(data.name.clone(), img.clone());
        }

        Self {
            main_font: server.load("fonts/FiraSans-Bold.ttf"),

            background: server.load("images/backg_2.png"),

            thumb_mesh: meshes.add(thumb_shape.into()),
            thumb_img,
        }
    }
}
impl AssetHandles for SongSelectAssetHandles {
    fn to_untyped_vec(&self) -> Vec<HandleUntyped> {
        let mut v = vec![
            self.main_font.clone_untyped(),
            self.background.clone_untyped(),
        ];
        v.extend(self.thumb_img.values().map(|img| img.clone_untyped()));
        v
    }
}
