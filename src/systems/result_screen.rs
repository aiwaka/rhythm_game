use bevy::prelude::*;

use crate::{
    components::{note::Note, ui::GameSceneObject},
    events::PanicAudio,
    resources::{
        game_scene::{AlreadyExistEntities, NextAppState, ResultDisplayed},
        handles::GameAssetsHandles,
        score::{CatchEval, ScoreResource, TimingEval},
        song::{AudioStartTime, SongConfig},
    },
    AppState, SCREEN_HEIGHT, SCREEN_WIDTH,
};

#[allow(clippy::too_many_arguments)]
fn spawn_result(
    mut commands: Commands,
    notes_q: Query<&Note>,
    song_config: Res<SongConfig>,
    start_time: Res<AudioStartTime>,
    time: Res<Time>,
    score: Res<ScoreResource>,
    handles: Res<GameAssetsHandles>,
    // すでに出現したかどうか
    spawned: Option<Res<ResultDisplayed>>,
    game_obj_q: Query<Entity, With<GameSceneObject>>,
    mut panic_audio_ev_writer: EventWriter<PanicAudio>,
) {
    if spawned.is_some() {
        return;
    }
    let time_after_start = time.seconds_since_startup() - start_time.0;
    let song_length = song_config.length;
    // ノーツが全部消えてかつ曲尺を2秒超えたらリザルト画面に移行
    if notes_q.is_empty() && song_length + 2.0 < time_after_start {
        // ゲームエンティティの片付け
        for ent in game_obj_q.iter() {
            commands.entity(ent).despawn_recursive();
        }
        // 音を停止
        panic_audio_ev_writer.send(PanicAudio);
        // リザルト表示
        commands.insert_resource(ResultDisplayed);
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::AZURE,
                custom_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        });
        // スコア表示テキストノード
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    border: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                color: UiColor(Color::NONE),
                ..Default::default()
            })
            .with_children(|parent| {
                let text = format!(
                    "Score: {}.\n\n\tPerfect: {}.\n\tOk: {}.\n\tMiss: {}.",
                    score.score(),
                    score.get_eval_num(&CatchEval::Perfect)
                        + score.get_eval_num(&CatchEval::NearPerfect(TimingEval::Fast))
                        + score.get_eval_num(&CatchEval::NearPerfect(TimingEval::Slow)),
                    score.get_eval_num(&CatchEval::Ok(TimingEval::Fast))
                        + score.get_eval_num(&CatchEval::Ok(TimingEval::Slow)),
                    score.get_eval_num(&CatchEval::Miss(TimingEval::Fast))
                        + score.get_eval_num(&CatchEval::Miss(TimingEval::Slow)),
                );
                parent.spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: text,
                            style: TextStyle {
                                font: handles.main_font.clone(),
                                font_size: 60.0,
                                color: Color::DARK_GRAY,
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    }
}

fn exit_game_state(
    mut commands: Commands,
    spawned: Option<Res<ResultDisplayed>>,
    mut key_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if spawned.is_some() && key_input.any_just_pressed([KeyCode::Z]) {
        key_input.reset_all();
        commands.remove_resource::<ResultDisplayed>();
        commands.insert_resource(NextAppState(AppState::SongSelect));
        state.set(AppState::Loading).unwrap()
    }
}

fn despawn_game_state(
    mut commands: Commands,
    already_exist: Res<AlreadyExistEntities>,
    entity_q: Query<Entity>,
) {
    for ent in entity_q.iter() {
        // もとからあったものではないエンティティをすべて削除する
        if !already_exist.0.contains(&ent) {
            commands.entity(ent).despawn();
        }
    }
    commands.remove_resource::<AlreadyExistEntities>();
    commands.remove_resource::<GameAssetsHandles>();
}

pub struct ResultScreenPlugin;
impl Plugin for ResultScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(spawn_result));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(exit_game_state));
        app.add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_game_state));
    }
}
