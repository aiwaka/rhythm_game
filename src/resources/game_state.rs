use crate::AppState;
use bevy::prelude::*;

#[derive(Resource)]
pub struct NextAppState(pub AppState);

/// エンティティ保存用リソース.
#[derive(Resource)]
pub struct ExistingEntities(pub Vec<Entity>);

#[derive(Resource)]
pub struct ResultDisplayed;

#[derive(Resource)]
pub struct GameCount(pub u32);

#[derive(Clone, Copy, Resource)]
pub enum GameDifficulty {
    Easy,
    Normal,
    Hard,
}
