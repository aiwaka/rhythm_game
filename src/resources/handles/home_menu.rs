use bevy::prelude::*;

use super::AssetHandles;

/// 曲セレクトシーンにおけるアセットハンドル
#[derive(Debug, Resource)]
pub struct HomeMenuAssetHandles {
    // フォント
    pub main_font: Handle<Font>,

    // 画像
    pub background: Handle<Image>,
}
impl HomeMenuAssetHandles {
    pub fn new(server: &Res<AssetServer>) -> Self {
        Self {
            main_font: server.load("fonts/FiraSans-Bold.ttf"),

            background: server.load("images/backg_2.png"),
        }
    }
}
impl AssetHandles for HomeMenuAssetHandles {
    fn to_untyped_vec(&self) -> Vec<HandleUntyped> {
        let v = vec![
            self.main_font.clone_untyped(),
            self.background.clone_untyped(),
        ];
        v
    }
}
