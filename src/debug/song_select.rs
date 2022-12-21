use bevy::prelude::*;

use crate::{
    events::PanicAudio,
    resources::{
        game_state::NextAppState,
        song::{SongConfigResource, SongNotes},
    },
    AppState,
};

/// リソース等を適切に片付けてから選曲画面に戻る機能
fn exit_game_state(
    mut commands: Commands,
    mut key_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut ev_writer: EventWriter<PanicAudio>,
) {
    if key_input.pressed(KeyCode::E) && key_input.just_pressed(KeyCode::B) {
        key_input.reset_all();
        ev_writer.send(PanicAudio);
        commands.remove_resource::<SongConfigResource>();
        commands.remove_resource::<SongNotes>();
        commands.insert_resource(NextAppState(AppState::SongSelect));
        state.set(AppState::Loading).unwrap()
    }
}

pub(super) struct DebugSongSelectPlugin;
impl Plugin for DebugSongSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(exit_game_state));
    }
}
