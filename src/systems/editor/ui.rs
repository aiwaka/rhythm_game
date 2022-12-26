use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{
    components::{
        note::KeyLane,
        timer::FrameCounter,
        ui::{ChartInfoNode, EditorStateObject, LaneLine, TargetLine},
    },
    constants::{LANE_WIDTH, TARGET_Y},
    resources::{config::GameDifficulty, handles::GameAssetsHandles, song::SongConfigResource},
    systems::system_labels::TimerSystemLabel,
    AppState,
};

fn setup_ui(
    mut commands: Commands,
    song_config: Res<SongConfigResource>,
    diff: Res<GameDifficulty>,
    handles: Res<GameAssetsHandles>,
) {
    // 曲名・難易度表示ノード
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .insert(EditorStateObject)
        .insert(ChartInfoNode)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                song_config.name.clone(),
                TextStyle {
                    font: handles.main_font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Edit mode".to_string(),
                TextStyle {
                    font: handles.main_font.clone(),
                    font_size: 20.0,
                    color: diff.get_color(),
                },
            ));
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
        .insert(EditorStateObject);

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
            .insert(EditorStateObject);
    }

    // DEBUG: 小節と拍表示用. EditorStateObjectなので終了時は勝手に消えてくれる
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Percent(80.0),
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .insert(EditorStateObject)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "init".to_string(),
                    TextStyle {
                        font: handles.main_font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ))
                .insert(BarBeatText);
        });
}
#[derive(Component)]
struct BarBeatText;

use crate::resources::editor::{EditorBar, EditorBeat};
fn debug_bb_text(
    mut q: Query<&mut Text, With<BarBeatText>>,
    current_bar: Res<EditorBar>,
    current_beat: Res<EditorBeat>,
) {
    for mut t in q.iter_mut() {
        t.sections[0].value = format!("{}:{}", **current_bar, **current_beat);
    }
}

fn setup_lane(mut commands: Commands, handles: Res<GameAssetsHandles>) {
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
            .insert(EditorStateObject)
            .insert(FrameCounter::new_default(60));
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

pub(super) struct EditorUiPlugin;
impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Editor).with_system(setup_ui));
        app.add_system_set(SystemSet::on_enter(AppState::Editor).with_system(setup_lane));
        app.add_system_set(SystemSet::on_update(AppState::Editor).with_system(debug_bb_text));
        app.add_system_set(
            SystemSet::on_update(AppState::Editor)
                .with_system(update_lane_background.after(TimerSystemLabel::FrameCounterUpdate)),
        );
    }
}
