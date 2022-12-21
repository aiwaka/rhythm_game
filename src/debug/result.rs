use bevy::prelude::*;

use crate::{
    resources::song::{SongConfigResource, SongNotes},
    AppState,
};

/// DEBUG: 強制的にノーツを残り0にしてリザルトに移行
fn debug_spawn_result(
    key_input: Res<Input<KeyCode>>,
    mut note_deque: ResMut<SongNotes>,
    mut song_config: ResMut<SongConfigResource>,
) {
    if key_input.just_pressed(KeyCode::R) && key_input.pressed(KeyCode::E) {
        note_deque.clear();
        song_config.length = 0.0;
    }
}

pub(super) struct DebugResultPlugin;
impl Plugin for DebugResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(debug_spawn_result));
    }
}
