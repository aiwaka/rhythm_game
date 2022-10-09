use bevy::prelude::*;

use super::{NotesPattern, PatternReceptor};
use crate::events::CatchNoteEvent;

/// 3列の右向き階段.
#[derive(Component)]
pub struct StepRightReceptor {
    is_initialized: bool,
    last_lane: i32,
    /// 最後に入力を許容した時刻
    last_time: f64,
    lane: [bool; 4],
}
impl Default for StepRightReceptor {
    fn default() -> Self {
        Self {
            is_initialized: true,
            last_lane: -1,
            last_time: 0.0,
            lane: [false; 4],
        }
    }
}

impl PatternReceptor for StepRightReceptor {
    fn init(&mut self) {
        self.is_initialized = true;
        self.last_lane = -1;
        self.lane = [false; 4];
    }

    fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    fn init_or_defer(&mut self, current_time: f64) {
        if (current_time - self.last_time).abs() > 0.4 {
            self.init();
        }
    }

    fn input(&mut self, note_ev: &CatchNoteEvent) {
        let column = note_ev.column;
        let real_sec = note_ev.real_sec;
        // 0, 1がfalseなら受付状態で, 0, 1が来たら開始
        if !self.lane[0] && !self.lane[1] && (column == 0 || column == 1) {
            self.last_time = real_sec;
            self.last_lane = column;
            self.lane[column as usize] = true;
            self.is_initialized = false;
            // 時刻が近すぎてもダメ
        } else if column == self.last_lane + 1 && real_sec - self.last_time > 0.01 {
            self.lane[column as usize] = true;
            self.last_time = real_sec;
            self.last_lane = column;
        }
    }

    // 3以上trueがあればOK
    fn achieved(&self) -> Option<NotesPattern> {
        (self.lane.iter().filter(|&&e| e).count() == 3).then_some(NotesPattern::StepRight)
    }

    fn is_available(&self) -> bool {
        true
    }
}
