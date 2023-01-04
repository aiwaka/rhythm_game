use std::io::Write;

use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    add_exit_system, add_update_system,
    components::{note::NoteInfo, ui::EditorStateObject},
    events::PanicAudio,
    resources::{
        editor::{EditorNotesQueue, QuittingEditor},
        game_state::NextAppState,
        handles::GameAssetsHandles,
        note::{NoteSpawn, NoteType},
        song::{SongConfig, SongConfigParser},
        song_list::SongData,
    },
    systems::load::{load_song_config, sort_spawn_notes},
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
fn merge_song_config(mut song_config: SongConfig, new_notes: Vec<NoteSpawn>) -> SongConfig {
    song_config.notes = new_notes;
    song_config
}

/// エディットモードをやめて保存するか聞く
fn quit_editting(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    handles: Res<GameAssetsHandles>,
    obj_q: Query<Entity, With<EditorStateObject>>,
    note_q: Query<Entity, With<NoteInfo>>,
    mut panic_audio_ev_writer: EventWriter<PanicAudio>,
) {
    if key_input.pressed(KeyCode::E) && key_input.just_pressed(KeyCode::Q) {
        // 音を停止
        panic_audio_ev_writer.send(PanicAudio);
        // エディタステートのオブジェクトを全て消去
        for ent in obj_q.iter() {
            commands.entity(ent).despawn_recursive();
        }
        for ent in note_q.iter() {
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
            let mut old_notes = song_config.notes.clone();
            let new_notes = queue
                .iter()
                .map(|n| NoteSpawn {
                    // TODO: ここでグリッドスナップ
                    note_type: NoteType::Normal { key: n.key },
                    bar: n.bar,
                    beat: n.beat,
                })
                .collect_vec();
            old_notes.extend(new_notes);
            let mut merged_notes = old_notes;
            sort_spawn_notes(&mut merged_notes);
            let merged_config = merge_song_config(song_config, merged_notes);
            // NOTE: 現状では（安定化するまで）バイナリの実行ディレクトリに吐き出される仕様となっている.
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
        add_update_system!(app, Editor, quit_editting);
        add_update_system!(app, Editor, back_to_home);
        add_exit_system!(app, Editor, exit_editor_state);
    }
}
