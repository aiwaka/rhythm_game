use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{
    components::{
        note::KeyLane,
        timer::FrameCounter,
        ui::{LaneLine, ScoreText, TargetLine, TimeText},
    },
    game_constants::{LANE_WIDTH, TARGET_POSITION},
    resources::{handles::GameAssetsHandles, score::ScoreResource, song::AudioStartTime},
    AppState,
};

use super::system_labels::TimerSystemLabel;

fn setup_ui(mut commands: Commands, handles: Res<GameAssetsHandles>) {
    let font = handles.main_font.clone();

    // 時間を表示するテキストノード
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::YELLOW_GREEN),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Time: 0.0".to_string(),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TimeText);
        });

    // スコア表示テキストノード
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
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
        translation: Vec3::new(0.0, TARGET_POSITION, 2.0),
        ..Default::default()
    };
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: Mesh2dHandle::from(handles.judge_line.clone()),
            material: handles.color_material_white_trans.clone(),
            transform,
            ..Default::default()
        })
        .insert(TargetLine);

    // 鍵盤線
    for i in 0..5 {
        let x = KeyLane::x_coord_from_num(i);
        let transform = Transform {
            translation: Vec3::new(x - LANE_WIDTH / 2.0, TARGET_POSITION + 250.0, 2.0),
            ..Default::default()
        };
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(handles.lane_line.clone()),
                material: handles.color_material_white_trans.clone(),
                transform,
                ..Default::default()
            })
            .insert(LaneLine);
    }
}

fn update_time_text(
    mut query: Query<(&mut Text, &TimeText)>,
    start_time: Res<AudioStartTime>,
    time: Res<Time>,
) {
    // Song starts 3 seconds after real time
    let time_after_start = time.seconds_since_startup() - start_time.0;

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
                "Score: {}. Corrects: {}. Fails: {}",
                score.score(),
                score.corrects(),
                score.fails()
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

pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_ui));
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(update_time_text.label(TimerSystemLabel::StartAudio)),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(update_score_text));
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(update_lane_background.after(TimerSystemLabel::FrameCounterUpdate)),
        );
    }
}
