pub mod all_sync;

use bevy::prelude::*;

use crate::events::CatchNoteEvent;

/// ノーツの並びパターン
#[derive(Clone, Copy, Debug)]
pub enum NotesPattern {
    Denim,
    /// 同時押し
    AllSync,
    /// 左上がり階段
    StepLeft,
    StepRight,
}

/// パターン受容体の機能を与えるトレイト.
/// 様々なノーツの配置パターンをキャッチできるようにするために機能を一般化する.
pub trait PatternReceptor: Default + Component {
    /// 初期化を行う. 何をもって初期化とするかはそれぞれに任せる.
    fn init(&mut self);

    /// 初期化されているかどうか（NOTE: 同期的でもないし使えない, 使っていない）
    fn is_initialized(&self) -> bool;

    /// 毎フレーム呼ばれる. 経過時刻等でリセットを行うか決める
    fn init_or_defer(&mut self, current_time: f64);

    /// ノーツを入力し状態を更新する. 適宜リセット等も行える.
    fn input(&mut self, note_ev: &CatchNoteEvent);

    /// 加点パターンの条件を満たしたかどうかを調べ, 満たしていたら対応するパターン列挙子を返す.
    fn achieved(&self) -> Option<NotesPattern>;

    /// 入力可能かどうかを返す.
    fn is_available(&self) -> bool;
}
