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

/// パターン受容体の機能を与えるトレイト
pub trait PatternReceptor: Default + Component {
    fn init(&mut self);

    fn is_initialized(&self) -> bool;

    /// 毎フレーム呼ばれる. リセットするか何もしないか
    fn init_or_defer(&mut self, current_time: f64);

    fn input(&mut self, note_ev: &CatchNoteEvent);

    fn achieved(&self) -> Option<NotesPattern>;

    fn is_available(&self) -> bool;
}
