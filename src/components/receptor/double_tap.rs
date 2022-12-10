use bevy::prelude::*;

use super::PatternReceptor;

#[derive(Component)]
pub struct DoubleTapReceptor {
    first_time: f64,
    lane: i32,
    num: u32,
}
impl Default for DoubleTapReceptor {
    fn default() -> Self {
        Self {
            first_time: 0.0,
            lane: -1,
            num: 0,
        }
    }
}
impl PatternReceptor for DoubleTapReceptor {
    fn init(&mut self) {
        *self = Self::default();
    }

    fn is_init(&self) -> bool {
        self.num == 0
    }

    fn init_or_defer(&mut self, current_time: f64, bpm: f32) {
        // 16分を少し超えたらリセット. 16.5 = 15 * 1.1によって少し猶予をもたせる
        if current_time - self.first_time > (bpm as f64).recip() * 16.5 {
            self.init();
        }
    }

    fn is_available(&self) -> bool {
        true
    }

    fn input(&mut self, note_ev: &crate::events::CatchNoteEvent) {
        if self.is_init() {
            self.first_time = note_ev.real_sec;
            self.lane = note_ev.column;
            self.num += 1;
        } else if self.lane == note_ev.column {
            self.num += 1;
        }
    }

    fn achieved(&self) -> Option<super::NotesPattern> {
        (self.num > 1).then_some(super::NotesPattern::DoubleTap)
    }
}
