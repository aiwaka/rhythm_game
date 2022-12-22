use bevy::prelude::*;

use crate::resources::note::NoteType;

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
    const NAME: &'static str = "Trill";

    #[cfg(feature = "debug")]
    fn debug_display(&self) -> String {
        let lane_str = self
            .lane
            .iter()
            .map(|l| {
                if *l == -1 {
                    "-".to_string()
                } else {
                    l.to_string()
                }
            })
            .collect::<String>();
        format!("{} : {}", lane_str, self.length)
    }

    fn initialized(&self) -> bool {
        self.length == 0
    }

    fn initialize_or_defer(&mut self, current_time: f64, bpm: f32) {
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
        if let NoteType::Normal { key } = note_ev.note.note_type {
            self.last_time = note_ev.real_time;
            if self.initialized() {
                self.lane[0] = key;
            } else if self.length == 1 {
                self.lane[1] = key;
                self.last_lane = 1;
            } else if self.lane[1 - self.last_lane] == key {
                self.last_lane = 1 - self.last_lane;
            } else {
                self.broken = true;
                return;
            }
            self.length += 1;
        }
    }

    fn is_available(&self) -> bool {
        true
    }

    fn achieved(&self) -> Option<NotesPattern> {
        // bool.then()によりOptionで包んだ値を返している
        (self.broken && self.length > 3).then(|| {
            if self.lane[0] == self.lane[1] {
                NotesPattern::MultipleTap(self.length)
            } else {
                NotesPattern::Trill(self.length)
            }
        })
    }
}
