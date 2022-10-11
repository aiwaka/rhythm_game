use bevy::prelude::*;

use super::{NotesPattern, PatternReceptor};

/// 3列トリル
#[derive(Component)]
pub struct StepTrillReceptor {
    lane: [i32; 3],
    /// 直前の入力のスロット番号. 0,1,2,1,0,1,...と遷移する
    last_lane: usize,
    /// スロット番号を遷移させるための変数. 1または-1をとる
    lane_move_direction: usize,
    last_time: f64,
    length: u32,
    /// トリルが切れたかどうかのフラグ. これがtrueになってlengthが一定以上なら加点とする
    broken: bool,
}
impl Default for StepTrillReceptor {
    fn default() -> Self {
        Self {
            lane: [-1; 3],
            last_lane: 0,
            lane_move_direction: 0,
            last_time: 0.0,
            length: 0,
            broken: true,
        }
    }
}

// impl PatternReceptor for StepTrillReceptor {
//     fn is_init(&self) -> bool {
//         self.length == 0
//     }

//     fn init_or_defer(&mut self, current_time: f64, bpm: f32) {
//         // 8分と少しの猶予（33.0 = 60 / 2 * 1.1）
//         if current_time - self.last_time > (bpm as f64).recip() * 33.0 {
//             // 初期化は行わず, フラグを立てる
//             self.broken = true;
//         }
//     }

//     fn input(&mut self, note_ev: &crate::events::CatchNoteEvent) {
//         let current_time = note_ev.real_sec;
//         let column = note_ev.column;
//         self.last_time = current_time;
//         if self.is_init() {
//             self.lane[0] = column;
//         } else if self.length == 1 {
//             self.lane[1] = column;
//             self.last_lane = 1;
//             self.broken = false;
//         } else if self.lane[1 - self.last_lane] == column {
//             self.length += 1;
//             self.last_lane = 1 - self.last_lane;
//         }
//     }

//     fn is_available(&self) -> bool {
//         true
//     }

//     fn achieved(&self) -> Option<NotesPattern> {
//         (self.broken && self.length > 4).then_some(NotesPattern::Trill(self.length))
//     }
// }
