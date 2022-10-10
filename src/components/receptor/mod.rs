mod denim;
mod full_sync;
mod step_left;
mod step_right;

/// レセプタ構造体を全部読み込むための公開モジュール
pub mod prelude {
    pub use super::{
        full_sync::FullSyncReceptor, step_left::StepLeftReceptor, step_right::StepRightReceptor,
    };
}

use bevy::prelude::*;

use crate::events::CatchNoteEvent;

/// ノーツの並びパターン
#[derive(Clone, Copy, Debug)]
pub enum NotesPattern {
    Denim,
    /// 同時押し
    FullSync,
    /// 左上がり階段
    StepLeft,
    StepRight,
}
impl std::fmt::Display for NotesPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match *self {
        //     _ => {}
        // }
        write!(f, "{:?}", self)
    }
}
impl NotesPattern {
    pub fn to_score(&self) -> u32 {
        match *self {
            NotesPattern::Denim => 2,
            _ => 1,
        }
    }
}

/// パターン受容体の機能を与えるトレイト.
/// 様々なノーツの配置パターンをキャッチできるようにするために機能を一般化する.
pub trait PatternReceptor: Default + Component {
    /// 初期化を行う. 何をもって初期化とするかはそれぞれに任せる.
    fn init(&mut self);

    /// 初期化されているかどうかを表す.
    fn is_init(&self) -> bool;

    /// 毎フレーム呼ばれる. 経過時刻等でリセットを行うか決める
    fn init_or_defer(&mut self, current_time: f64, bpm: f32);

    /// ノーツを入力し状態を更新する. 適宜リセット等も行える.
    fn input(&mut self, note_ev: &CatchNoteEvent);

    /// 加点パターンの条件を満たしたかどうかを調べ, 満たしていたら対応するパターン列挙子を返す.
    fn achieved(&self) -> Option<NotesPattern>;

    /// 入力可能かどうかを返す.
    fn is_available(&self) -> bool;
}
