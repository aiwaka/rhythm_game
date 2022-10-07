use bevy::prelude::*;

use crate::{
    components::audio::AudioStartTimer, events::AudioStartEvent,
    resources::handles::GameAssetsHandles, AppState,
};

use super::system_labels::TimerSystemLabel;

fn setup_start_song(mut commands: Commands) {
    commands
        .spawn()
        .insert(AudioStartTimer(Timer::from_seconds(3.0, false)));
}

fn start_song(
    mut commands: Commands,
    audio: Res<Audio>,
    mut timer_query: Query<(&mut AudioStartTimer, Entity)>,
    time: Res<Time>,
    handles: Res<GameAssetsHandles>,
    mut ev_writer: EventWriter<AudioStartEvent>,
) {
    if let Ok((mut timer, ent)) = timer_query.get_single_mut() {
        timer.0.tick(time.delta());
        // ゲーム開始から3秒後にスタート
        if timer.0.finished() {
            info!("music start");
            audio.play(handles.music.clone());
            ev_writer.send(AudioStartEvent);
            commands.entity(ent).despawn();
        }
    }
}

pub struct GameAudioPlugin;
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_start_song));
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(start_song.label(TimerSystemLabel::StartAudio)),
        );
    }
}
