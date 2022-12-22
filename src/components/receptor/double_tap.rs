use bevy::prelude::*;

use crate::resources::note::NoteType;

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
            // 縦連中に他のレーンが挟まると途切れてしまうが, 仕様ということにする
            // 2鍵以上の縦連の場合は他のレセプタに任せるか, これを拡張する
            first_time: 0.0,
            lane: -1,
            num: 0,
        }
    }
}
impl PatternReceptor for DoubleTapReceptor {
    const NAME: &'static str = "DoubleTap";

    #[cfg(feature = "debug")]
    fn debug_display(&self) -> String {
        if self.initialized() {
            "init".to_string()
        } else {
            "accepting".to_string()
        }
    }

    fn init(&mut self) {
        *self = Self::default();
    }

    fn initialized(&self) -> bool {
        self.num == 0
    }

    fn initialize_or_defer(&mut self, current_time: f64, bpm: f32) {
        // 16分を少し超えたらリセット. 16.5 = 15 * 1.1によって少し猶予をもたせる
        if current_time - self.first_time > (bpm as f64).recip() * 16.5 {
            self.init();
        }
    }

    fn is_available(&self) -> bool {
        true
    }

    fn input(&mut self, note_ev: &crate::events::CatchNoteEvent) {
        if let NoteType::Normal { key } = note_ev.note.note_type {
            if self.initialized() {
                self.first_time = note_ev.real_time;
                self.lane = key;
                self.num += 1;
            } else if self.lane == key {
                self.num += 1;
            }
        }
    }

    fn achieved(&self) -> Option<super::NotesPattern> {
        (self.num > 1).then_some(super::NotesPattern::DoubleTap)
    }
}
