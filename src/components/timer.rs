use std::ops::Deref;

use bevy::prelude::*;

/// 初期化したフレーム数減少するカウントダウン
#[derive(Component)]
pub struct CountDownTimer {
    count: u32,
    pause: bool,
    auto_despawn: bool,
}
impl CountDownTimer {
    /// newでつくったコンポーネントをくっつけたエンティティはカウントが終わると自動で破棄される
    pub fn new(count: u32) -> Self {
        Self {
            count,
            pause: false,
            auto_despawn: true,
        }
    }
    /// 自動でエンティティを削除されないコンポーネントとして作成する（このコンポーネント自体は終了時取り除かれる）
    pub fn new_will_not_be_removed(count: u32) -> Self {
        Self {
            count,
            pause: false,
            auto_despawn: false,
        }
    }
    pub fn tick(&mut self) {
        if !self.pause && self.count > 0 {
            self.count -= 1;
        }
    }
    #[inline]
    pub fn count(&self) -> u32 {
        self.count
    }
    #[inline]
    pub fn is_finished(&self) -> bool {
        self.count == 0
    }
    pub fn stop(&mut self) {
        self.pause = true;
    }
    #[inline]
    pub fn is_paused(&self) -> bool {
        self.pause
    }
    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }
    #[inline]
    pub fn auto_despawn(&self) -> bool {
        self.auto_despawn
    }
}

/// tickを呼ぶと1増加するカウンター
#[derive(Component, Clone, Debug)]
pub struct FrameCounter {
    count: u32,
    pause: bool,
}
impl FrameCounter {
    pub fn new() -> Self {
        Self {
            count: 0,
            pause: false,
        }
    }
    pub fn tick(&mut self) {
        if !self.pause {
            self.count += 1;
        }
    }
    pub fn reset(&mut self) {
        self.count = 0;
    }
    #[inline]
    #[must_use]
    pub fn is_pause(&self) -> bool {
        self.pause
    }
    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }
    #[inline]
    pub fn count(&self) -> u32 {
        self.count
    }
}

// デリファレンスでカウントを取得できるように
impl Deref for FrameCounter {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.count
    }
}

#[derive(Component)]
pub struct PlayerShotTimer(pub Timer);

impl PlayerShotTimer {
    pub fn new() -> Self {
        PlayerShotTimer(Timer::from_seconds(0.1, true))
    }
    pub fn reset(&mut self) {
        self.0 = Timer::from_seconds(0.1, true);
    }
}

#[derive(Component)]
pub struct PlayerAnimationTimer(pub Timer);
impl PlayerAnimationTimer {
    pub fn new() -> Self {
        PlayerAnimationTimer(Timer::from_seconds(0.3, true))
    }
}

// 持続的にダメージを与える場合毎フレーム処理すると大変なことになるのでそのためのタイマーをつくる
#[derive(Component)]
pub struct ContinuousDamageTimer(pub Timer);
impl ContinuousDamageTimer {
    pub fn new() -> Self {
        ContinuousDamageTimer(Timer::from_seconds(0.2, true))
    }
}
