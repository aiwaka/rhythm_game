use std::io::prelude::*;
use std::{collections::VecDeque, fs::File};

use bevy::{asset::LoadState, prelude::*};
use itertools::Itertools;

use crate::resources::handles::HomeMenuAssetHandles;
use crate::{add_enter_system, add_exit_system, add_update_system};
use crate::{
    components::{load::NowLoadingText, note::NoteInfo},
    constants::{BASIC_NOTE_SPEED, DISTANCE},
    resources::{
        config::{Beat, Bpm, GameDifficulty, NoteSpeed},
        game_state::NextAppState,
        handles::{AssetHandles, AssetsLoading, GameAssetsHandles, SongSelectAssetHandles},
        note::{NoteSpawn, NoteType},
        score::ScoreResource,
        song::{SongConfig, SongConfigParser, SongConfigResource, SongNotes},
        song_list::{AllSongData, SongData, SongDataParser},
    },
    AppState,
};

/// 曲一覧情報をファイルから取得する.
fn load_all_config_file_data() -> Vec<SongDataParser> {
    let mut file = File::open("assets/songs/all_song_data.yaml").expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file into String");

    let parsed: Vec<SongDataParser> =
        serde_yaml::from_str(&contents).expect("Couldn't parse into data array");
    parsed
}

/// 指定された曲情報ファイルの内容を返す
pub(super) fn load_song_config(filename: &str) -> SongConfig {
    let mut file = File::open(format!("assets/songs/{}", filename)).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file into String");

    let parsed: SongConfigParser =
        serde_yaml::from_str(&contents).expect("Couldn't parse into SongConfigParser");

    SongConfig::from(parsed)
}

/// NoteSpawnの列を小節と拍によりソートする.
pub fn sort_spawn_notes(notes: &mut [NoteSpawn]) {
    notes.sort_by(|a, b| match a.bar.cmp(&b.bar) {
        std::cmp::Ordering::Equal => a.beat.partial_cmp(&b.beat).unwrap(),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
    });
}

/// barとbeatのみの構造の列からspawn_timeとtarget_timeを持った構造の列に変換する
pub fn to_notes_info_from_notes_spawn(
    mut spawn_notes: Vec<NoteSpawn>,
    speed: f32,
    initial_bpm: f32,
    initial_beat: u32,
) -> Vec<NoteInfo> {
    // ノーツをソートする.
    sort_spawn_notes(&mut spawn_notes);

    // ノーツを配列に収める
    #[allow(unused_mut)]
    let mut beat_par_bar = initial_beat; // 拍子
    #[allow(unused_mut)]
    let mut bpm = initial_bpm;
    // 判定線への到達タイムを蓄積させる変数
    // 途中でBPMや拍子を変更するようなイベントがあればそれを反映する.
    // 判定線に到達する時間を曲開始時刻から測ったもの.
    let mut target_time = 0.0;
    let mut notes = vec![];
    let mut prev_beat = 0.0;

    let mut prev_bar = 0u32;
    for note in spawn_notes {
        // このような仕様のため, 拍子を変更する場合は小節の最初に行い, かつbeat_diffの計算の前に行う.
        // その上で, 前の拍から変更前の小節が終わるまで何拍か記憶しておき, 次の拍に足し合わせる作業が必要.
        let beat_diff = if note.bar == prev_bar {
            note.beat - prev_beat
        } else {
            // 小節番号の差を追加
            let bar_diff = note.bar - prev_bar;
            prev_bar = note.bar;
            note.beat + (bar_diff * beat_par_bar) as f64 - prev_beat
        };
        target_time += beat_diff * (bpm as f64).recip() * 60.0;
        let spawn_time = target_time - ((DISTANCE / speed) as f64).abs();
        notes.push(NoteInfo {
            note_type: note.note_type.clone(),
            target_time,
            spawn_time,
            bar: note.bar,
            beat: note.beat,
        });
        prev_beat = note.beat;
    }
    notes
}

/// 指定された曲情報ファイルから曲の情報を持ったリソースを返す.
fn load_song_config_resources(
    filename: &str,
    speed_coeff: f32,
    diff: &GameDifficulty,
) -> (SongConfigResource, SongNotes, Bpm, Beat) {
    // cloneが不要になるよう全部バラしてから再構成する
    let SongConfig {
        name,
        filename,
        length,
        initial_beat,
        initial_bpm,
        notes: mut config_notes,
    } = load_song_config(filename);

    let song_config_resource = SongConfigResource {
        name,
        song_filename: filename,
        length,
    };
    // 小節線ノートを加える
    let last_bar_num = if let Some(note) = config_notes.iter().last() {
        note.bar
    } else {
        0
    };
    for bar in 0..(last_bar_num + 2) {
        config_notes.push(NoteSpawn {
            note_type: NoteType::BarLine,
            bar,
            beat: 0.0,
        })
    }

    let mut notes = to_notes_info_from_notes_spawn(
        config_notes,
        speed_coeff * BASIC_NOTE_SPEED,
        initial_bpm,
        initial_beat,
    );

    // Master難易度でない場合はアドリブノーツを削除する
    if !matches!(*diff, GameDifficulty::Master) {
        notes = notes
            .into_iter()
            .filter(|note| !matches!(note.note_type, NoteType::AdLib { key: _ }))
            .collect_vec();
    }

    (
        song_config_resource,
        SongNotes(VecDeque::from_iter(notes)),
        Bpm(initial_bpm),
        Beat(initial_beat),
    )
}

/// アセットのロードを開始する.
/// また, 各シーンに移行したときに用意されているべきリソース等を準備する.
#[allow(clippy::too_many_arguments)]
fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    next_scene: Res<NextAppState>,
    mut color_material: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    selected_song: Option<Res<SongData>>,
    speed: Option<Res<NoteSpeed>>,
    diff: Option<Res<GameDifficulty>>,
) {
    // 型なしのアセット列を用意
    let mut assets_loading_vec = Vec::<HandleUntyped>::new();

    // 次がどのシーンに行くかによって分岐.
    match next_scene.0 {
        AppState::HomeMenu => {
            let assets = HomeMenuAssetHandles::new(&asset_server);
            assets_loading_vec.extend(assets.to_untyped_vec());
            commands.insert_resource(assets);
        }
        AppState::SongSelect => {
            // 全曲データを読み込む
            let parsed_data = load_all_config_file_data();
            let data = parsed_data.into_iter().map(SongData::from).collect_vec();

            let assets =
                SongSelectAssetHandles::new(&asset_server, &mut texture_atlas, &mut meshes, &data);
            // 読み込んだハンドルを型を外してクローンした配列をもらう.
            assets_loading_vec.extend(assets.to_untyped_vec());
            commands.insert_resource(assets);

            commands.insert_resource(AllSongData(data));
            // 難易度をここで用意しておく（選択画面でもゲーム中でも共用する）
            commands.insert_resource(GameDifficulty::Normal);
        }
        AppState::Game | AppState::Editor => {
            // ゲームステートに遷移する前にはこれらのリソースを用意しておかなければならない.
            let selected_song = selected_song.unwrap();
            let speed = speed.unwrap();

            // 曲データをロード
            let (config, notes, bpm, beat) = load_song_config_resources(
                &selected_song.config_file_name,
                speed.0,
                &diff.unwrap(),
            );
            let music_filename = config.song_filename.clone();
            commands.insert_resource(config);
            commands.insert_resource(notes);
            commands.insert_resource(bpm);
            commands.insert_resource(beat);

            let assets = GameAssetsHandles::new(
                music_filename,
                &asset_server,
                &mut texture_atlas,
                &mut color_material,
                &mut meshes,
            );
            // 読み込んだハンドルを型を外してクローンした配列をもらう.
            assets_loading_vec.extend(assets.to_untyped_vec());
            commands.insert_resource(assets);

            // スコアリソースを初期化
            commands.insert_resource(ScoreResource::default());
        }
        _ => {}
    }
    // ローディング中の型無しアセットとしてリソースに追加
    commands.insert_resource(AssetsLoading(assets_loading_vec));
    // ローディング中テキストエンティティを出現させる.
    #[cfg(feature = "debug")]
    let loading_text = "Now Loading...(Debug Mode)".to_string();
    #[cfg(not(feature = "debug"))]
    let loading_text = "Now Loading...".to_string();
    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(20.0),
                    right: Val::Px(40.0),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                loading_text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        })
        .insert(NowLoadingText);
}

fn check_assets_ready(
    mut state: ResMut<State<AppState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    next_scene: Res<NextAppState>,
) {
    // すべてロードが終わったかどうかを確認してから次のシーンへ移行する
    match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        // ここでローディングテキストや画像を動かしてもいい.
        LoadState::Loading => {}
        LoadState::Failed => {
            warn!("loading failed");
        }
        LoadState::Loaded => {
            info!("loaded");
            // ロード完了したら次のシーンに遷移する命令
            state.set(next_scene.0).unwrap();
        }
        _ => {}
    }
}

fn exit_loading(mut commands: Commands, text_q: Query<Entity, With<NowLoadingText>>) {
    // ロード完了を確認したのでロード用一時ハンドル列を削除する
    commands.remove_resource::<AssetsLoading>();
    // 次のステートの情報も削除する.
    commands.remove_resource::<NextAppState>();
    // ローディング文字列も消去
    if let Ok(ent) = text_q.get_single() {
        commands.entity(ent).despawn();
    }
}

pub struct LoadPlugin;
impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        // アセットロード関連システム
        add_enter_system!(app, Loading, load_assets);
        add_update_system!(app, Loading, check_assets_ready);
        add_exit_system!(app, Loading, exit_loading);
    }
}
