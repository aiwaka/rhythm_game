use bevy::prelude::*;

use crate::{
    components::{note::NoteInfo, result_screen::ScrollingList, ui::GameSceneObject},
    events::PanicAudio,
    resources::{
        game_state::{ExistingEntities, NextAppState, ResultDisplayed},
        handles::GameAssetsHandles,
        score::{CatchEval, ScoreResource, TimingEval},
        song::{SongConfigResource, SongStartTime},
    },
    AppState, SCREEN_HEIGHT, SCREEN_WIDTH,
};

#[allow(clippy::too_many_arguments)]
fn spawn_result(
    mut commands: Commands,
    notes_q: Query<&NoteInfo>,
    song_config: Res<SongConfigResource>,
    start_time: Res<SongStartTime>,
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
    let time_after_start = time.elapsed_seconds_f64() - start_time.0;
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
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::AZURE,
                custom_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        });
        // スコア表示テキストノード
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    border: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            })
            .with_children(|parent| {
                let text = format!(
                    "Score: {}.\n\n\tPerfect: {}.\n\tOk: {}.\n\tMiss: {}.",
                    score.get_score(),
                    score.get_eval_num(&CatchEval::Perfect)
                        + score.get_eval_num(&CatchEval::NearPerfect(TimingEval::Fast))
                        + score.get_eval_num(&CatchEval::NearPerfect(TimingEval::Slow)),
                    score.get_eval_num(&CatchEval::Ok(TimingEval::Fast))
                        + score.get_eval_num(&CatchEval::Ok(TimingEval::Slow)),
                    score.get_eval_num(&CatchEval::Miss) + score.get_eval_num(&CatchEval::Miss),
                );
                parent.spawn(TextBundle {
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

        let pattern_vec = score.get_pattern_vec();
        if !pattern_vec.is_empty() {
            // 画面右に寄せる
            commands
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(
                        TextBundle::from_section(
                            "Achieved Pattern",
                            TextStyle {
                                font: handles.main_font.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            size: Size::new(Val::Undefined, Val::Px(30.0)),
                            margin: UiRect {
                                left: Val::Auto,
                                right: Val::Auto,
                                ..default()
                            },
                            ..default()
                        }),
                    );
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                                overflow: Overflow::Hidden,
                                ..default()
                            },
                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        flex_grow: 1.0,
                                        max_size: Size::UNDEFINED,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(ScrollingList::default())
                                .with_children(|parent| {
                                    // ここで達成したパターンをくっつける
                                    for pat in pattern_vec {
                                        parent.spawn(
                                            TextBundle::from_section(
                                                pat.to_string(),
                                                TextStyle {
                                                    font: handles.main_font.clone(),
                                                    font_size: 30.0,
                                                    color: Color::WHITE,
                                                },
                                            )
                                            .with_style(Style {
                                                flex_shrink: 0.0,
                                                size: Size::new(Val::Undefined, Val::Px(20.0)),
                                                margin: UiRect {
                                                    left: Val::Auto,
                                                    right: Val::Auto,
                                                    ..default()
                                                },
                                                ..default()
                                            }),
                                        );
                                    }
                                });
                        });
                });
        }
    }
}

fn scroll_pattern_list(
    key_input: Res<Input<KeyCode>>,
    mut list_q: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    item_q: Query<&Node>,
) {
    if key_input.any_pressed([KeyCode::Down, KeyCode::Up]) {
        for (mut scrolling_list, mut style, children, uinode) in &mut list_q {
            let items_height = children
                .iter()
                .map(|entity| item_q.get(*entity).unwrap().size().y)
                .sum::<f32>();
            let panel_height = uinode.size().y;
            // リストの高さ合計が枠の高さをはみ出した分+αだけスクロール可能
            let max_scroll = (items_height - panel_height + 20.0).max(0.0);
            let dy = if key_input.pressed(KeyCode::Up) {
                4.0
            } else if key_input.pressed(KeyCode::Down) {
                -4.0
            } else {
                0.0
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.0);
            // 最終的なスクロール位置にセット
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}

fn exit_game_state(
    mut commands: Commands,
    spawned: Option<Res<ResultDisplayed>>,
    mut key_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if spawned.is_some() && key_input.any_just_pressed([KeyCode::Z, KeyCode::Return]) {
        key_input.reset_all();
        commands.remove_resource::<ResultDisplayed>();
        commands.insert_resource(NextAppState(AppState::SongSelect));
        state.set(AppState::Loading).unwrap()
    }
}

fn despawn_game_state(
    mut commands: Commands,
    already_exist: Res<ExistingEntities>,
    entity_q: Query<Entity>,
) {
    for ent in entity_q.iter() {
        // もとからあったものではないエンティティをすべて削除する
        if !already_exist.0.contains(&ent) {
            commands.entity(ent).despawn();
        }
    }
    commands.remove_resource::<ExistingEntities>();
    commands.remove_resource::<GameAssetsHandles>();
}

pub struct ResultScreenPlugin;
impl Plugin for ResultScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(spawn_result));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(scroll_pattern_list));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(exit_game_state));
        app.add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_game_state));
    }
}
