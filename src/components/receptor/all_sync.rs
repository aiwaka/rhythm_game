use bevy::prelude::*;

use super::{NotesPattern, PatternReceptor};
use crate::events::CatchNoteEvent;

#[derive(Component)]
pub struct AllSyncReceptor {
    is_initialized: bool,
    first_time: f64,
    lane: [bool; 4],
}
impl Default for AllSyncReceptor {
    fn default() -> Self {
        Self {
            is_initialized: true,
            first_time: 0.0,
            lane: [false; 4],
        }
    }
}

impl PatternReceptor for AllSyncReceptor {
    fn init(&mut self) {
        self.is_initialized = true;
        self.lane = [false; 4];
    }

    fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    fn init_or_defer(&mut self, current_time: f64) {
        if (current_time - self.first_time).abs() > 0.1 {
            self.init();
        }
    }

    fn input(&mut self, note_ev: &CatchNoteEvent) {
        info!("note: {:?}", note_ev);
        let column = note_ev.column;
        let real_sec = note_ev.real_sec;
        if self.is_initialized {
            self.first_time = real_sec;
            self.lane[column as usize] = true;
            self.is_initialized = false;
        } else {
            // 初期状態で追加された瞬間から0.1秒経過したらリセット
            if real_sec - self.first_time > 0.1 {
                self.init();
                return;
            }
            self.lane[column as usize] = true;
        }
    }

    fn achieved(&self) -> Option<super::NotesPattern> {
        self.lane
            .iter()
            .all(|e| *e)
            .then_some(NotesPattern::AllSync)
    }

    fn is_available(&self) -> bool {
        true
    }
}
