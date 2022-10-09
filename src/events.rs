//! ゲームで使うイベント構造体をここで定義する.
use bevy::prelude::*;

use crate::components::{note::Note, receptor::NotesPattern};

/// ノーツをキャッチできたときに発されるイベント
#[derive(Clone, Debug)]
pub struct CatchNoteEvent {
    /// 鍵盤番号
    pub column: i32,
    /// 小節番号
    pub bar: u32,
    /// 拍番号
    pub beat: f64,
    /// 曲開始からの経過時間（理論値）
    pub exact_sec: f64,
    /// 実際に取得された時間
    pub real_sec: f64,
    /// bpm
    pub bpm: f32,
    /// 一小節の拍数
    pub beat_par_bar: u32,
}
impl CatchNoteEvent {
    pub fn new(note: &Note, real_sec: f64, bpm: f32, beat_par_bar: u32) -> Self {
        Self {
            column: note.key_column,
            bar: note.bar,
            beat: note.beat,
            exact_sec: note.target_time,
            real_sec,
            bpm,
            beat_par_bar,
        }
    }
}

/// ノーツ配置パターンを完成させたときに発されるイベント.
#[derive(Clone, Debug)]
pub struct AchievePatternEvent(pub NotesPattern);

/// 追加したイベントをappに追加する処理をここでまとめて行う.
pub(super) fn add_events_to_game(app: &mut App) {
    app.add_event::<CatchNoteEvent>();
    app.add_event::<AchievePatternEvent>();
}
