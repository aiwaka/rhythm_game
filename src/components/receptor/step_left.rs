use bevy::prelude::*;

use super::{NotesPattern, PatternReceptor};
use crate::{events::CatchNoteEvent, game_constants::ERROR_THRESHOLD};

/// 3列の右向き階段.
#[derive(Component)]
pub struct StepLeftReceptor {
    last_lane: i32,
    /// 最後に入力を許容した時刻
    last_time: f64,
    lane: [bool; 4],
}
impl Default for StepLeftReceptor {
    fn default() -> Self {
        Self {
            last_lane: -1,
            last_time: 0.0,
            lane: [false; 4],
        }
    }
}

impl PatternReceptor for StepLeftReceptor {
    fn init(&mut self) {
        self.last_lane = -1;
        self.lane = [false; 4];
    }

    fn is_init(&self) -> bool {
        !self.lane[2] && !self.lane[3]
    }

    fn init_or_defer(&mut self, current_time: f64, bpm: f32) {
        if (current_time - self.last_time).abs()
            > bpm.recip() as f64 * 60.0 + ERROR_THRESHOLD as f64
        {
            self.init();
        }
    }

    fn input(&mut self, note_ev: &CatchNoteEvent) {
        let column = note_ev.column;
        let real_sec = note_ev.real_sec;
        // 2, 3がfalseなら受付状態で, 2, 3が来たら開始
        if self.is_init() && (column == 2 || column == 3) {
            self.last_time = real_sec;
            self.last_lane = column;
            self.lane[column as usize] = true;
            // 時刻が近すぎてもダメ
        } else if column == self.last_lane - 1 && real_sec - self.last_time > 0.01 {
            self.lane[column as usize] = true;
            self.last_time = real_sec;
            self.last_lane = column;
        }
    }

    // 3以上trueがあればOK
    fn achieved(&self) -> Option<NotesPattern> {
        (self.lane.iter().filter(|&&e| e).count() == 3).then_some(NotesPattern::StepLeft)
    }

    fn is_available(&self) -> bool {
        true
    }
}
