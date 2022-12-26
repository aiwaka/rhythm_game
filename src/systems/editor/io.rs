use std::io::Write;

use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    components::{note::NoteInfo, ui::EditorStateObject},
    constants::BASIC_NOTE_SPEED,
    events::PanicAudio,
    resources::{
        config::NoteSpeed,
        editor::{EditorNotesQueue, QuittingEditor},
        game_state::NextAppState,
        handles::GameAssetsHandles,
        note::NoteSpawn,
        song::{SongConfig, SongConfigParser},
        song_list::SongData,
    },
    systems::load::{load_song_config, to_notes_info_from_notes_spawn},
    AppState,
};

/// 最終的なファイル出力を行う
pub(super) fn output_chart(filename: &str, config: SongConfig) -> Result<(), std::io::Error> {
    let mut file = match std::fs::File::create(filename) {
        Err(why) => panic!("couldn't open : {}", why),
        Ok(file) => file,
    };
    let parser = SongConfigParser::from(config);
    let data_str = serde_yaml::to_string(&parser).expect("cannot parse into yaml data.");
    file.write_all(data_str.as_bytes())
}

/// もとの譜面情報のノーツ情報を出力したいノーツ列で上書きしたデータを返す
fn merge_song_config(mut song_config: SongConfig, new_notes: Vec<NoteInfo>) -> SongConfig {
    song_config.notes = new_notes
        .iter()
        .map(|n| NoteSpawn {
            note_type: n.note_type.clone(),
            bar: n.bar,
            beat: n.beat,
        })
        .collect_vec();
    song_config
}

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
    queue: Res<EditorNotesQueue>,
    speed_coeff: Res<NoteSpeed>,
    song_data: Res<SongData>,
) {
    if quitting.is_some()
        && key_input.pressed(KeyCode::E)
        && key_input.any_just_pressed([KeyCode::S, KeyCode::D])
    {
        // 保存する場合は追加の操作
        if key_input.just_pressed(KeyCode::S) {
            // 新しく譜面データを読み出し
            let song_config = load_song_config(&song_data.config_file_name);
            let notes = to_notes_info_from_notes_spawn(
                song_config.notes.clone(),
                speed_coeff.0 * BASIC_NOTE_SPEED,
                song_config.initial_bpm,
                song_config.initial_beat,
            );
            let merged_notes = convert_raw_input_notes(&queue, &notes);
            let merged_config = merge_song_config(song_config, merged_notes);
            output_chart(&song_data.config_file_name, merged_config).unwrap();
        }
        key_input.clear();
        commands.insert_resource(NextAppState(AppState::HomeMenu));
        state.set(AppState::Loading).unwrap();
    }
}

/// on_exitでの処理
fn exit_editor_state(mut commands: Commands, obj_q: Query<Entity, With<EditorStateObject>>) {
    for ent in obj_q.iter() {
        commands.entity(ent).despawn_recursive();
    }
    commands.remove_resource::<EditorNotesQueue>();
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
