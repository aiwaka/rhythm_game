use bevy::prelude::*;

use crate::{constants::LANE_WIDTH, resources::note::NoteType};

/// ゲームが使う情報を入れた構造体. 取得時の受け渡しのためコンポーネントとして使う.
#[derive(Component, Debug, Clone)]
pub struct NoteInfo {
    pub note_type: NoteType,
    pub bar: u32,
    pub beat: f64,
    pub spawn_time: f64,
    pub target_time: f64,
}

/// 鍵盤レーン
#[derive(Component, Clone, Copy, Debug)]
pub struct KeyLane(pub i32);
impl KeyLane {
    /// 鍵盤の数
    pub const KEY_NUM: u8 = 4;

    /// 番号から入力可能キーを返す
    fn get_key_list(&self) -> [KeyCode; 3] {
        match self.0 {
            0 => [KeyCode::C, KeyCode::D, KeyCode::S],
            1 => [KeyCode::V, KeyCode::F, KeyCode::G],
            2 => [KeyCode::N, KeyCode::J, KeyCode::H],
            3 => [KeyCode::M, KeyCode::K, KeyCode::L],
            _ => [KeyCode::Return, KeyCode::Return, KeyCode::Return],
        }
    }
    /// 番号とキーを結びつけ, 指定された鍵盤番号に対応するキーが今押されたかどうかを取得.
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        input.any_just_pressed(self.get_key_list())
    }
    pub fn key_pressed(&self, input: &Input<KeyCode>) -> bool {
        input.any_pressed(self.get_key_list())
    }
    pub fn key_just_released(&self, input: &Input<KeyCode>) -> bool {
        input.any_just_released(self.get_key_list())
    }
    /// 複数の入力が可能な鍵盤に対して, 確実に全ての入力をリセットする.
    pub fn reset_key(&self, input: &mut Input<KeyCode>) {
        let keys = self.get_key_list();
        for key in keys {
            input.reset(key);
        }
    }

    /// x_coordをi32から取得
    pub fn x_coord_from_num(num: i32) -> f32 {
        let half_width = LANE_WIDTH / 2.0;
        0.0 - (Self::KEY_NUM - 1) as f32 * half_width + LANE_WIDTH * num as f32
    }
}

#[derive(Component)]
pub struct MissingNote;

/// ロングノーツの状態を表す
#[derive(Debug, Default)]
pub enum LongNoteState {
    /// 未取得, 未処理
    #[default]
    BeforeRetrieve,
    /// ホールド中
    Hold,
    /// 取得失敗
    Miss,
    /// 正常終了
    End,
}
/// ロングノーツにつけて演出を行う.
#[derive(Component, Default)]
pub struct LongNote {
    pub state: LongNoteState,
}
impl LongNote {
    pub fn new() -> Self {
        Self::default()
    }
}
