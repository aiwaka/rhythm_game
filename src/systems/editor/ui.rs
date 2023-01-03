use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{
    components::{
        editor::BarBeatText,
        note::KeyLane,
        timer::FrameCounter,
        ui::{ChartInfoNode, EditorStateObject, LaneLine, TargetLine},
    },
    constants::{LANE_WIDTH, TARGET_Y},
    resources::{config::GameDifficulty, handles::GameAssetsHandles, song::SongConfigResource},
    spawn_text_node,
    systems::system_labels::TimerSystemLabel,
    AppState,
};

fn setup_ui(
    mut commands: Commands,
    song_config: Res<SongConfigResource>,
    diff: Res<GameDifficulty>,
    handles: Res<GameAssetsHandles>,
) {
    let font = handles.main_font.clone();
    // 曲名・難易度表示ノード
    spawn_text_node!(
        commands,
        font,
        [left : 10.0, top : 10.0],
        Color::NONE,
        [
            [song_config.name.clone(), 30.0, Color::WHITE, []],
            ["Edit mode", 20.0, diff.get_color(), []]
        ],
        [EditorStateObject, ChartInfoNode]
    );

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

    // 小節と拍表示用. EditorStateObjectなので終了時は勝手に消えてくれる
    spawn_text_node!(
        commands,
        font,
        [left: 600.0, top: 10.0],
        Color::NONE,
        [[
            "init",
            30.0,
            Color::WHITE,
            [BarBeatText]
        ]],
        [EditorStateObject],
        { size: Size::new(Val::Auto, Val::Px(30.0)) }
    );
}

use crate::resources::editor::{EditorBar, EditorBeat};
fn beat_and_bar_text(
    mut q: Query<&mut Text, With<BarBeatText>>,
    current_bar: Res<EditorBar>,
    current_beat: Res<EditorBeat>,
) {
    for mut t in q.iter_mut() {
        t.sections[0].value = format!("{:>03}:{:>07.4}", **current_bar, **current_beat);
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
        app.add_system_set(SystemSet::on_update(AppState::Editor).with_system(beat_and_bar_text));
        app.add_system_set(
            SystemSet::on_update(AppState::Editor)
                .with_system(update_lane_background.after(TimerSystemLabel::FrameCounterUpdate)),
        );
    }
}
