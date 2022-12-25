use bevy::prelude::*;

use crate::{
    components::{note::NoteInfo, ui::EditorStateObject},
    events::PanicAudio,
    resources::{
        editor::{EditorNotesQueue, QuittingEditor},
        game_state::NextAppState,
        handles::GameAssetsHandles,
    },
    AppState,
};

pub(super) fn output_chart(filename: &str, notes: &[NoteInfo]) {}

/// 時刻情報のみ持っている入力されたノーツ列と他のイベント（BPM変更等含む）を組み合わせてパース可能な列にする
pub(super) fn convert_raw_input_notes(
    queue: &EditorNotesQueue,
    notes: &[NoteInfo],
) -> Vec<NoteInfo> {
    vec![]
}

/// エディットモードをやめて保存するか聞く
fn quit_editting(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    handles: Res<GameAssetsHandles>,
    obj_q: Query<Entity, With<EditorStateObject>>,
    mut panic_audio_ev_writer: EventWriter<PanicAudio>,
) {
    if key_input.pressed(KeyCode::E) && key_input.just_pressed(KeyCode::Q) {
        // 音を停止
        panic_audio_ev_writer.send(PanicAudio);
        // エディタステートのオブジェクトを全て消去
        for ent in obj_q.iter() {
            commands.entity(ent).despawn_recursive();
        }

        // 終了状態を表すリソースを追加
        commands.insert_resource(QuittingEditor);
        // テキストノード
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    border: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::AZURE),
                ..Default::default()
            })
            .insert(EditorStateObject)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Quit Editor\nE+S => Save this edit\nE+D => Discard this edit",
                        TextStyle {
                            font: handles.main_font.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..Default::default()
                });
            });
    }
}

/// 保存するまたはしないを決めて戻る処理. 保存の有無でシステムを分けると同時に実行されると危ない
fn back_to_home(
    mut commands: Commands,
    quitting: Option<Res<QuittingEditor>>,
    mut key_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if quitting.is_some() && key_input.pressed(KeyCode::E) {
        if key_input.just_pressed(KeyCode::S) {
            commands.insert_resource(NextAppState(AppState::HomeMenu));
            key_input.clear();
            state.set(AppState::Loading).unwrap();
        } else if key_input.just_pressed(KeyCode::D) {
            info!("discard");
            commands.insert_resource(NextAppState(AppState::HomeMenu));
            key_input.clear();
            state.set(AppState::Loading).unwrap();
        }
    }
}

/// on_exitでの処理
fn exit_editor_state(mut commands: Commands, obj_q: Query<Entity, With<EditorStateObject>>) {
    for ent in obj_q.iter() {
        commands.entity(ent).despawn_recursive();
    }
    commands.remove_resource::<QuittingEditor>();
}

pub(super) struct EditorInOutPlugin;
impl Plugin for EditorInOutPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Editor).with_system(quit_editting));
        app.add_system_set(SystemSet::on_update(AppState::Editor).with_system(back_to_home));
        app.add_system_set(SystemSet::on_exit(AppState::Editor).with_system(exit_editor_state));
    }
}
