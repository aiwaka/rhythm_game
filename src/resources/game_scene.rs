use crate::AppState;
use bevy::prelude::Entity;

pub struct NextAppState(pub AppState);

/// エンティティ保存用リソース.
pub struct AlreadyExistEntities(pub Vec<Entity>);

pub struct ResultDisplayed;

pub struct GameCount(pub u32);

#[derive(Clone, Copy)]
pub enum GameDifficulty {
    Easy,
    Normal,
    Hard,
}
