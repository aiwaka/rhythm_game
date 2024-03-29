use bevy::prelude::*;

use super::{NotesPattern, PatternReceptor};
use crate::{events::CatchNoteEvent, resources::note::NoteType};

/// 鍵盤にノーツを収めるにはbool配列を使うが, usizeのインデックスと鍵盤番号i32の対応を行う必要がある.
/// そのため鍵盤番号が連番なのを前提として最小の番号がどれになるかを指定する.
const KEY_FIRST_NUM: i32 = 0;

/// 4点同時押し
#[derive(Component)]
pub struct FullSyncReceptor {
    first_time: f64,
    lane: [bool; 4],
}
impl Default for FullSyncReceptor {
    fn default() -> Self {
        Self {
            first_time: 0.0,
            lane: [false; 4],
        }
    }
}

impl PatternReceptor for FullSyncReceptor {
    const NAME: &'static str = "FullSync";

    #[cfg(feature = "debug")]
    fn debug_display(&self) -> String {
        use crate::debug::utilities::boolean_string;

        boolean_string(&self.lane)
    }

    #[inline(always)]
    fn init(&mut self) {
        self.lane = [false; 4];
    }

    fn initialized(&self) -> bool {
        self.lane.iter().all(|&e| !e)
    }

    fn initialize_or_defer(&mut self, current_time: f64, bpm: f32) {
        // 最初の受付から1/6拍の時間経過でリセット
        if (current_time - self.first_time).abs() > 10.0 / bpm as f64 {
            self.init();
        }
    }

    fn input(&mut self, note_ev: &CatchNoteEvent) {
        if let NoteType::Normal { key } | NoteType::AdLib { key } = note_ev.note.note_type {
            let real_sec = note_ev.real_time;
            // 鍵盤番号を0始まりのインデックスに変換する
            let idx = (key - KEY_FIRST_NUM) as usize;
            if self.initialized() {
                self.first_time = real_sec;
            }
            self.lane[idx] = true;
        }
    }

    fn achieved(&self) -> Option<super::NotesPattern> {
        self.lane
            .iter()
            .all(|e| *e)
            .then_some(NotesPattern::FullSync)
    }
}
