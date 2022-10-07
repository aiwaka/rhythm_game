use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    game_constants::MUSIC_PLAY_PRECOUNT,
    resources::{handles::GameAssetsHandles, note::AudioStartTime},
    AppState,
};

use super::system_labels::TimerSystemLabel;

fn setup_start_song(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(AudioStartTime(
        time.seconds_since_startup() + MUSIC_PLAY_PRECOUNT,
    ));
}

fn start_song(
    audio: Res<Audio>,
    start_time: Res<AudioStartTime>,
    time: Res<Time>,
    handles: Res<GameAssetsHandles>,
) {
    // 曲開始時刻から現在時刻までの差
    let time_after_start = time.seconds_since_startup() - start_time.0;
    let time_last = time_after_start - time.delta_seconds_f64();
    if time_last < 0.0 && 0.0 < time_after_start {
        info!("music start");
        audio.play(handles.music.clone());
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
