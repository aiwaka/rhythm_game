use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    add_enter_system, add_update_system,
    constants::MUSIC_PLAY_PRECOUNT,
    events::PanicAudio,
    resources::{handles::GameAssetsHandles, song::SongStartTime},
    AppState,
};

use super::system_labels::TimerSystemLabel;

fn setup_start_song(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(SongStartTime(
        time.elapsed_seconds_f64() + MUSIC_PLAY_PRECOUNT,
    ));
}

fn start_song(
    audio: Res<Audio>,
    start_time: Res<SongStartTime>,
    time: Res<Time>,
    handles: Res<GameAssetsHandles>,
) {
    // 曲開始時刻から現在時刻までの差
    let time_after_start = start_time.time_after_start(&time);
    let time_last = time_after_start - time.delta_seconds_f64();
    if (time_last..time_after_start).contains(&0.0) {
        info!("music start");
        audio.play(handles.music.clone());
    }
}

fn setup_editor_start_song(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(SongStartTime(
        time.elapsed_seconds_f64() + MUSIC_PLAY_PRECOUNT,
    ));
}
fn editor_start_song(
    audio: Res<Audio>,
    start_time: Res<SongStartTime>,
    time: Res<Time>,
    handles: Res<GameAssetsHandles>,
) {
    let time_after_start = start_time.time_after_start(&time);
    let time_last = time_after_start - time.delta_seconds_f64();
    if (time_last..time_after_start).contains(&0.0) {
        info!("editor music start");
        audio.play(handles.music.clone());
    }
}

fn panic_audio(audio: Res<Audio>, ev_reader: EventReader<PanicAudio>) {
    if !ev_reader.is_empty() {
        audio.stop();
    }
}

pub struct GameAudioPlugin;
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        add_enter_system!(app, Game, setup_start_song);
        add_update_system!(app, Game, start_song, [], TimerSystemLabel::StartAudio);
        add_enter_system!(app, Editor, setup_editor_start_song);
        add_update_system!(
            app,
            Editor,
            editor_start_song,
            [],
            TimerSystemLabel::StartAudio
        );
        app.add_system(panic_audio);
    }
}
