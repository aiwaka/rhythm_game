use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{
    components::{
        note::KeyLane,
        ui::{ChartInfoNode, EditorSceneObject, GameSceneObject, LaneLine, TargetLine},
    },
    constants::{LANE_WIDTH, TARGET_Y},
    resources::{
        config::GameDifficulty,
        handles::GameAssetsHandles,
        song::{SongConfigResource, SongStartTime},
    },
    AppState, SCREEN_HEIGHT, SCREEN_WIDTH,
};

fn setup_ui(
    mut commands: Commands,
    song_config: Res<SongConfigResource>,
    diff: Res<GameDifficulty>,
    handles: Res<GameAssetsHandles>,
) {
    let font = handles.main_font.clone();
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
        .insert(EditorSceneObject)
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

pub(super) struct EditorUiPlugin;
impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Editor).with_system(setup_ui));
    }
}
