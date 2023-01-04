use std::ops::Deref;

use bevy::prelude::*;

/// 1フレームに1減少するカウントダウンタイマー.
/// カウントが0になると, くっつけられているエンティティが自動で削除される.
/// タイマーが終わったことを使うSystemには.after(TimerSystemLabel::TimerUpdate)の指定が必要.
#[derive(Component, Debug)]
pub struct CountDownTimer {
    count: u32,
    pause: bool,
    auto_despawn: bool,
}
impl CountDownTimer {
    /// カウントを指定してタイマーを生成する.
    /// newでつくったコンポーネントをくっつけたエンティティはカウントが終わると自動で再帰的に破棄（despawn_recursive）される
    pub fn new(count: u32) -> Self {
        Self {
            count,
            pause: false,
            auto_despawn: true,
        }
    }
    /// 終了時に自動でエンティティを削除しないタイマーを作成する.
    /// （このタイマーコンポーネント自体は終了時に取り除かれる）
    pub fn new_not_despawn(count: u32) -> Self {
        Self {
            count,
            pause: false,
            auto_despawn: false,
        }
    }
    /// カウントを1減らす.
    pub fn tick(&mut self) {
        if !self.pause && self.count > 0 {
            self.count -= 1;
        }
    }
    /// 現在の残りカウントを取得する.
    #[inline]
    pub fn count(&self) -> u32 {
        self.count
    }
    /// タイマーが終了しているかどうか取得する.
    #[inline]
    pub fn is_finished(&self) -> bool {
        self.count == 0
    }
    /// タイマーを一時停止する.
    pub fn pause(&mut self) {
        self.pause = true;
    }
    /// 一時停止状態かどうかを取得する.
    #[must_use]
    #[inline]
    pub fn paused(&self) -> bool {
        self.pause
    }
    /// 一時停止状態を切り替える.
    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }
    /// 自動でエンティティを破棄するタイマーであるかどうかを取得する.
    #[inline]
    pub fn auto_despawn(&self) -> bool {
        self.auto_despawn
    }
}

/// 1フレームに1増えるカウンタ. 一時停止可能.
#[derive(Component, Clone, Debug)]
pub struct FrameCounter {
    count: u32,
    pause: bool,
}
impl FrameCounter {
    /// 0から始まるカウンタを生成する.
    pub fn new() -> Self {
        Self::new_default(0)
    }
    /// 任意のカウントから始まるカウンタを生成する.
    pub fn new_default(count: u32) -> Self {
        Self {
            count,
            pause: false,
        }
    }
    /// カウントを1進める
    pub fn tick(&mut self) {
        if !self.pause {
            self.count += 1;
        }
    }
    /// カウントを0に戻す
    pub fn reset(&mut self) {
        self.count = 0;
    }
    /// 一時停止状態かどうかを取得する.
    #[inline]
    #[must_use]
    pub fn paused(&self) -> bool {
        self.pause
    }
    /// 一時停止状態を切り替える.
    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }
    /// カウントを取得する.
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
