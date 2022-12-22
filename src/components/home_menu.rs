use bevy::prelude::Component;

/// ホームメニューにおけるエンティティ
#[derive(Component)]
pub struct HomeMenuObject;

#[derive(Component, Debug, Clone)]
pub struct ActiveOption(pub usize);
#[derive(Component, Debug, Clone)]
pub struct HomeMenuOptionItem(pub usize);

#[derive(Component, Debug, Clone, Copy)]
pub enum HomeMenuOption {
    Start,
    Exit,
}
