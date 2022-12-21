use bevy::prelude::*;

use super::{NotesPattern, PatternReceptor};
use crate::{events::CatchNoteEvent, resources::note::NoteType};

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
    fn init(&mut self) {
        self.lane = [false; 4];
    }

    fn initialized(&self) -> bool {
        self.lane.iter().all(|&e| !e)
    }

    fn initialize_or_defer(&mut self, current_time: f64, _: f32) {
        if (current_time - self.first_time).abs() > 0.1 {
            self.init();
        }
    }

    fn input(&mut self, note_ev: &CatchNoteEvent) {
        if let NoteType::Normal { key } = note_ev.note.note_type {
            let real_sec = note_ev.real_time;
            if self.initialized() {
                self.first_time = real_sec;
                // TODO: keyをusizeに変換するのでkeyがi32の意味がない. 鍵盤の使い方を検討
                self.lane[key as usize] = true;
            } else {
                self.lane[key as usize] = true;
            }
        }
    }

    fn achieved(&self) -> Option<super::NotesPattern> {
        self.lane
            .iter()
            .all(|e| *e)
            .then_some(NotesPattern::FullSync)
    }

    fn is_available(&self) -> bool {
        true
    }
}
