use bevy::prelude::*;

/// 曲選択UIの親ノードを示すマーカー
#[derive(Component)]
pub struct SongSelectParentNode;

#[derive(Component)]
pub struct SongSelectText;

/// リストにくっつける, 現在選ばれている番号を示す
#[derive(Component)]
pub struct ActiveSongCard(pub usize);

/// 曲カード. カード番号を保存
#[derive(Component)]
pub struct SongSelectCard(pub usize);

#[derive(Component)]
pub struct DifficultyText;
