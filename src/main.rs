mod components;
mod constants;
mod events;
mod resources;
mod systems;

#[cfg(feature = "debug")]
mod debug;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use events::add_events_to_game;
use resources::game_state::NextAppState;
use systems::{
    audio::GameAudioPlugin, load::LoadPlugin, note::NotePlugin, receptor::PatternReceptorPlugin,
    result_screen::ResultScreenPlugin, score::ScorePlugin, song_select::SongSelectStatePlugin,
    timer::TimersPlugin, ui::GameUiPlugin,
};

#[cfg(feature = "debug")]
use debug::AppDebugPlugin;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    HomeMenu,
    SongSelect,
    Loading,
    Game,
}

fn global_setup(mut commands: Commands) {
    // カメラのセット
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    let window = WindowDescriptor {
        title: "rhythm".to_string(),
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        resizable: true,
        ..Default::default()
    };

    let mut app = App::new();

    // Set antialiasing to use 4 samples
    app.insert_resource(Msaa { samples: 4 });
    app.add_system(bevy::window::close_on_esc);
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window,
        ..Default::default()
    }));
    app.add_plugin(AudioPlugin);
    // ステート初期化
    // 次に向かいたいステートをセットしてからローディングステートで開始する.
    app.insert_resource(NextAppState(AppState::SongSelect));
    app.add_state(AppState::Loading);

    add_events_to_game(&mut app);

    app.add_startup_system(global_setup);
    app.add_plugin(LoadPlugin);
    app.add_plugin(NotePlugin);
    app.add_plugin(GameUiPlugin);
    app.add_plugin(GameAudioPlugin);
    app.add_plugin(TimersPlugin);
    app.add_plugin(PatternReceptorPlugin);
    app.add_plugin(ScorePlugin);

    app.add_plugin(SongSelectStatePlugin);
    app.add_plugin(ResultScreenPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(AppDebugPlugin);
    // app.add_plugin(ShadersPlugin);
    app.run();
}
