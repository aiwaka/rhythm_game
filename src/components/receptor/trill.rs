use bevy::prelude::*;

use super::{NotesPattern, PatternReceptor};

/// 2列トリル
#[derive(Component, Debug)]
pub struct TrillReceptor {
    lane: [i32; 2],
    /// 直前の入力がスロットの0,1どちらか
    last_lane: usize,
    last_time: f64,
    length: u32,
    /// トリルが切れたかどうかのフラグ. これがtrueになってlengthが一定以上なら加点とする
    broken: bool,
}
impl Default for TrillReceptor {
    fn default() -> Self {
        Self {
            lane: [-1; 2],
            last_lane: 0,
            last_time: 0.0,
            length: 0,
            broken: false,
        }
    }
}

impl PatternReceptor for TrillReceptor {
    fn is_init(&self) -> bool {
        self.length == 0
    }

    fn init_or_defer(&mut self, current_time: f64, bpm: f32) {
        if self.broken {
            self.init();
        }
        // 8分と少しの猶予（33.0 = 60 / 2 * 1.1）
        else if current_time - self.last_time > (bpm as f64).recip() * 33.0 {
            // 初期化は行わず, フラグを立てる
            self.broken = true;
        }
    }

    fn input(&mut self, note_ev: &crate::events::CatchNoteEvent) {
        let column = note_ev.column;
        self.last_time = note_ev.real_sec;
        if self.is_init() {
            self.lane[0] = column;
        } else if self.length == 1 {
            self.lane[1] = column;
            self.last_lane = 1;
        } else if self.lane[1 - self.last_lane] == column {
            self.last_lane = 1 - self.last_lane;
        } else {
            self.broken = true;
        }
        self.length += 1;
    }

    fn is_available(&self) -> bool {
        true
    }

    fn achieved(&self) -> Option<NotesPattern> {
        (self.broken && self.length > 4).then_some(NotesPattern::Trill(self.length))
    }
}
