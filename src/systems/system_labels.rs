//! システムの順序などに用いるラベルを定義する.
use bevy::prelude::SystemLabel;

#[derive(SystemLabel)]
pub(super) enum TimerSystemLabel {
    /// タイマーが終了したことを用いるシステムは.after()でこれをつけなければうまくいかない.
    TimerUpdate,
    FrameCounterUpdate,
    UpdateGameCount,
}

pub(super) enum AnimeEffectSystemLabel {
    Animate,
}

#[derive(SystemLabel)]
pub(super) enum BoardInfoSystemLabel {
    Update,
}

/// 描画用のTransformにPositionを反映するシステムのラベル
#[derive(SystemLabel)]
pub(super) struct ReflectTransform;
