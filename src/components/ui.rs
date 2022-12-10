use bevy::prelude::*;

/// ゲームの中で使われるエンティティに付与
#[derive(Component)]
pub struct GameSceneObject;

#[derive(Component)]
pub struct TimeText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct TargetLine;

#[derive(Component)]
pub struct LaneLine;

/// レーンに対応するキーの名前を表示するテキスト.
#[derive(Component)]
pub struct LaneKeyText;

/// 鍵盤レーンの背景. キー押下時に色を出したり.
#[derive(Component)]
pub struct LaneBackground(pub i32);

#[derive(Component)]
pub struct PatternPopupText;

#[derive(Component)]
pub struct CatchEvalPopupText;
