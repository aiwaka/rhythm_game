use bevy::{prelude::*, sprite::Mesh2dHandle};
use itertools::Itertools;
use rand::Rng;

use crate::{
    components::{
        note::KeyLane,
        timer::{CountDownTimer, FrameCounter},
        ui::{
            CatchEvalPopupText, GameSceneObject, LaneLine, PatternPopupText, ScoreText, TargetLine,
            TimeText,
        },
    },
    constants::{LANE_WIDTH, TARGET_Y},
    events::{AchievePatternEvent, CatchNoteEvent},
    resources::{
        game_state::ExistingEntities,
        handles::GameAssetsHandles,
        note::NoteType,
        score::{CatchEval, ScoreResource, TimingEval},
        song::SongStartTime,
    },
    AppState, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::system_labels::{PatternReceptorSystemLabel, TimerSystemLabel, UiSystemLabel};

fn setup_ui(mut commands: Commands, handles: Res<GameAssetsHandles>) {
    let font = handles.main_font.clone();

    // スコア表示テキストノード
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .insert(GameSceneObject)
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Score: 0. Corrects: 0. Fails: 0".to_string(),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.8, 0.8, 0.8),
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ScoreText);
        });
    // 判定線
    let transform = Transform {
        translation: Vec3::new(0.0, TARGET_Y, 2.0),
        ..Default::default()
    };
    commands
        .spawn(ColorMesh2dBundle {
            mesh: Mesh2dHandle::from(handles.judge_line.clone()),
            material: handles.color_material_white_trans.clone(),
            transform,
            ..Default::default()
        })
        .insert(TargetLine)
        .insert(GameSceneObject);

    // 鍵盤線
    for i in 0..5 {
        let x = KeyLane::x_coord_from_num(i);
        let transform = Transform {
            translation: Vec3::new(x - LANE_WIDTH / 2.0, TARGET_Y + 250.0, 2.0),
            ..Default::default()
        };
        commands
            .spawn(ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(handles.lane_line.clone()),
                material: handles.color_material_white_trans.clone(),
                transform,
                ..Default::default()
            })
            .insert(LaneLine)
            .insert(GameSceneObject);
    }
}

fn setup_lane(
    mut commands: Commands,
    handles: Res<GameAssetsHandles>,
    already_exist_q: Query<Entity>,
) {
    // シーン遷移時点で存在しているエンティティをすべて保存
    commands.insert_resource(ExistingEntities(already_exist_q.iter().collect_vec()));
    for i in 0..4 {
        let x = KeyLane::x_coord_from_num(i);
        let transform = Transform {
            translation: Vec3::new(x, TARGET_Y + 250.0, 0.1),
            ..Default::default()
        };
        commands
            .spawn(ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(handles.lane_background.clone()),
                material: handles.color_material_lane_background[i as usize].clone(),
                transform,
                ..Default::default()
            })
            .insert(KeyLane(i))
            .insert(FrameCounter::new_default(60));
    }
}

fn update_time_text(
    mut query: Query<(&mut Text, &TimeText)>,
    start_time: Res<SongStartTime>,
    time: Res<Time>,
) {
    // Song starts 3 seconds after real time
    let time_after_start = time.elapsed_seconds_f64() - start_time.0;

    // Don't do anything before the song starts
    if time_after_start < 0.0 {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.2}", time_after_start);
    }
}

fn update_score_text(score: Res<ScoreResource>, mut query: Query<(&mut Text, &ScoreText)>) {
    if score.is_changed() {
        for (mut text, _marker) in query.iter_mut() {
            text.sections[0].value = format!(
                "Score: {}. Perfect: {}. Ok: {}. Miss: {}.",
                score.score(),
                score.get_eval_num(&CatchEval::Perfect)
                    + score.get_eval_num(&CatchEval::NearPerfect(TimingEval::Fast))
                    + score.get_eval_num(&CatchEval::NearPerfect(TimingEval::Slow)),
                score.get_eval_num(&CatchEval::Ok(TimingEval::Fast))
                    + score.get_eval_num(&CatchEval::Ok(TimingEval::Slow)),
                score.get_eval_num(&CatchEval::Miss)
            );
        }
    }
}

fn update_lane_background(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Handle<ColorMaterial>, &KeyLane, &mut FrameCounter)>,
    key_input: Res<Input<KeyCode>>,
) {
    for (color, lane, mut counter) in query.iter_mut() {
        if lane.key_just_pressed(&key_input) {
            counter.reset();
        }
        let opacity = 1.0 - counter.count().clamp(0, 10) as f32 / 10.0;
        let new_color = &mut materials.get_mut(color).unwrap().color;
        new_color.set_a(opacity);
    }
}

fn spawn_pattern_text(
    mut commands: Commands,
    mut ev_reader: EventReader<AchievePatternEvent>,
    handles: Res<GameAssetsHandles>,
) {
    // 乱数生成器
    let mut rng = rand::thread_rng();

    let font = handles.main_font.clone();
    for ev in ev_reader.iter() {
        // 出現位置をある程度ランダムに
        #[allow(clippy::if_same_then_else)]
        let pos_x: f32 = if rng.gen_bool(0.5) {
            rng.gen_range(10.0..=40.0)
        } else {
            rng.gen_range(580.0..=620.0)
        };
        let pos_y: f32 = rng.gen_range(200.0..=300.0);
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(pos_x),
                        top: Val::Px(pos_y),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            })
            .insert(CountDownTimer::new(30))
            .insert(PatternPopupText)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("{}", ev.0),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::YELLOW,
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    }
}

fn update_pattern_text(
    mut node_q: Query<(&mut Style, &CountDownTimer, &Children), With<PatternPopupText>>,
    mut text_q: Query<&mut Text>,
) {
    for (mut style, timer, children) in node_q.iter_mut() {
        if !timer.is_finished() {
            for &child in children.iter() {
                if let Ok(mut text) = text_q.get_mut(child) {
                    let opacity = timer.count().clamp(0, 20) as f32 / 20.0;
                    let color = &mut text.sections[0].style.color;
                    color.set_a(opacity);
                    if let Val::Px(ref mut prev_pos) = style.position.top {
                        *prev_pos -= 3.0;
                    }
                }
            }
        }
    }
}

/// ノーツ取得評価テキストを出現させる
fn spawn_catch_eval_text(
    mut commands: Commands,
    mut ev_reader: EventReader<CatchNoteEvent>,
    handles: Res<GameAssetsHandles>,
) {
    let font = handles.main_font.clone();
    for ev in ev_reader.iter() {
        let Some(Vec2 {x: pos_left, y: pos_bottom}) = (match ev.note.note_type {
            NoteType::Normal { key } => {
                Some(Vec2::new(SCREEN_WIDTH / 2.0 + KeyLane::x_coord_from_num(key) - LANE_WIDTH / 2.0, SCREEN_HEIGHT / 2.0 + TARGET_Y))
            }
            NoteType::AdLib { key } => {
                Some(Vec2::new(SCREEN_WIDTH / 2.0 + KeyLane::x_coord_from_num(key) - LANE_WIDTH / 2.0, SCREEN_HEIGHT / 2.0 + TARGET_Y))
            }
            NoteType::BarLine => None,
        }) else { continue };
        let catch_eval = CatchEval::new(ev.note.target_time, ev.real_sec);
        commands
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(LANE_WIDTH), Val::Px(50.0)),
                    position: UiRect {
                        left: Val::Px(pos_left),
                        bottom: Val::Px(pos_bottom),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            })
            .insert(CountDownTimer::new(15))
            .insert(CatchEvalPopupText)
            .with_children(|parent| {
                if let Some(timing) = catch_eval.get_timing() {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("{}", timing),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 15.0,
                                    color: timing.get_color(),
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                }
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("{}", catch_eval),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 30.0,
                                color: catch_eval.get_color(),
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    }
}

fn update_catch_eval_text(
    mut node_q: Query<(&mut Style, &CountDownTimer, &Children), With<CatchEvalPopupText>>,
    mut text_q: Query<&mut Text>,
) {
    for (mut style, timer, children) in node_q.iter_mut() {
        if !timer.is_finished() {
            for &child in children.iter() {
                if let Ok(mut text) = text_q.get_mut(child) {
                    let opacity = timer.count().clamp(0, 10) as f32 / 10.0;
                    let color_ref = &mut text.sections[0].style.color;
                    color_ref.set_a(opacity);
                    if let Val::Px(ref mut prev_pos) = style.position.top {
                        *prev_pos -= 3.0;
                    }
                }
            }
        }
    }
}

pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_ui));
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_lane));
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(update_time_text.label(TimerSystemLabel::StartAudio)),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(update_score_text));
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(update_lane_background.after(TimerSystemLabel::FrameCounterUpdate)),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                spawn_pattern_text
                    .label(UiSystemLabel::SpawnPatternText)
                    .after(TimerSystemLabel::TimerUpdate)
                    .after(PatternReceptorSystemLabel::Recept),
            ),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                update_pattern_text
                    .after(TimerSystemLabel::TimerUpdate)
                    .after(UiSystemLabel::SpawnPatternText),
            ),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_catch_eval_text.after(TimerSystemLabel::TimerUpdate)),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(update_catch_eval_text.after(TimerSystemLabel::TimerUpdate)),
        );
    }
}
