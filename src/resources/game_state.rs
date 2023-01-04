use crate::AppState;
use bevy::prelude::*;

#[derive(Resource)]
pub struct NextAppState(pub AppState);

/// エンティティ保存用リソース.
#[derive(Resource)]
pub struct ExistingEntities(pub Vec<Entity>);

/// 存在していればリザルト画面にいることを表す.
#[derive(Resource)]
pub struct ResultDisplayed;
