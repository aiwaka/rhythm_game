use bevy::prelude::*;

/// 選曲画面でエディット不可能な譜面を編集しようとした際にアラートを出すためのコンポーネント.
#[derive(Component)]
pub struct FrozenChartErrorText;

#[derive(Component)]
pub struct BarBeatText;
