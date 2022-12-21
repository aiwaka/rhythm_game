use bevy::prelude::*;

use super::{NotesPattern, PatternReceptor};
use crate::{constants::MISS_THR, events::CatchNoteEvent, resources::note::NoteType};

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
    const NAME: &'static str = "StepLeft";
    fn init(&mut self) {
        self.last_lane = -1;
        self.lane = [false; 4];
    }

    fn initialized(&self) -> bool {
        !self.lane[2] && !self.lane[3]
    }

    fn initialize_or_defer(&mut self, current_time: f64, bpm: f32) {
        if (current_time - self.last_time).abs() > bpm.recip() as f64 * 60.0 + MISS_THR {
            self.init();
        }
    }

    fn input(&mut self, note_ev: &CatchNoteEvent) {
        if let NoteType::Normal { key } = note_ev.note.note_type {
            let real_sec = note_ev.real_time;
            // 2, 3がfalseなら受付状態で, 2, 3が来たら開始
            if self.initialized() && (key == 2 || key == 3) {
                self.last_time = real_sec;
                self.last_lane = key;
                self.lane[key as usize] = true;
                // 時刻が近すぎてもダメ
            } else if key == self.last_lane - 1 && real_sec - self.last_time > 0.01 {
                self.lane[key as usize] = true;
                self.last_time = real_sec;
                self.last_lane = key;
            }
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
