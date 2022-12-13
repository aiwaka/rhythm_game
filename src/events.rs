//! ゲームで使うイベント構造体をここで定義する.
use bevy::prelude::*;

use crate::{
    components::{note::NoteInfo, receptor::NotesPattern},
    resources::score::CatchEval,
};

// /// ノーツを取り逃したときのイベント
// #[derive(Clone, Copy, Debug)]
// pub struct MissNoteEvent;

/// ノーツ取得評価を送信するイベント
#[derive(Clone, Debug, Deref)]
pub struct NoteEvalEvent(pub CatchEval);

/// ノーツをキャッチできたときに発されるイベント
#[derive(Clone, Debug)]
pub struct CatchNoteEvent {
    pub note: NoteInfo,
    /// 実際に取得された時間
    pub real_sec: f64,
    /// bpm
    pub bpm: f32,
    /// 一小節の拍数
    pub beat: u32,
}
impl CatchNoteEvent {
    pub fn new(note: &NoteInfo, real_sec: f64, bpm: f32, beat: u32) -> Self {
        Self {
            note: note.clone(),
            real_sec,
            bpm,
            beat,
        }
    }
}

/// ノーツ配置パターンを完成させたときに発されるイベント.
#[derive(Clone, Debug)]
pub struct AchievePatternEvent(pub NotesPattern);

/// すべての音を止めるイベント
pub struct PanicAudio;

/// 追加したイベントをappに追加する処理をここでまとめて行う.
pub(super) fn add_events_to_game(app: &mut App) {
    app.add_event::<CatchNoteEvent>();
    // app.add_event::<MissNoteEvent>();
    app.add_event::<NoteEvalEvent>();
    app.add_event::<AchievePatternEvent>();
    app.add_event::<PanicAudio>();
}
