use bevy::prelude::*;

#[derive(Component)]
pub struct TimeText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct TargetLine;

#[derive(Component)]
pub struct LaneLine;

/// 鍵盤レーンの背景. キー押下時に色を出したり.
#[derive(Component)]
pub struct LaneBackground(pub i32);
