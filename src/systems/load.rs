use bevy::{asset::LoadState, prelude::*};
use itertools::Itertools;

use crate::{
    components::{load::NowLoadingText, note::NoteInfo},
    constants::{BASIC_NOTE_SPEED, DISTANCE},
    resources::{
        config::{Beat, Bpm, NoteSpeed},
        game_state::NextAppState,
        handles::{AssetHandles, AssetsLoading, GameAssetsHandles, SongSelectAssetHandles},
        note::{NoteSpawn, NoteType},
        score::ScoreResource,
        song::{SongConfig, SongConfigParser, SongConfigResource, SongNotes},
        song_list::{AllSongData, SongData, SongDataParser},
    },
    AppState,
};
use std::io::prelude::*;
use std::{collections::VecDeque, fs::File};

/// 曲一覧情報を取得する.
/// TODO: 現在ハードコーディングしているが, tomlファイルから読み込むように変更する.
fn load_all_config_file_data() -> Vec<SongDataParser> {
    vec![
        SongDataParser {
            name: "Hot Tide".to_string(),
            thumbnail: 0,
            config_file_name: "hot_tide.yaml".to_string(),
        },
        SongDataParser {
            name: "Abraxas".to_string(),
            thumbnail: 0,
            config_file_name: "abraxas.yaml".to_string(),
        },
        // SongDataParser {
        //     name: "Autoseeker".to_string(),
        //     thumbnail: 0,
        //     config_file_name: "hot_tide.toml".to_string(),
        // },
        // SongDataParser {
        //     name: "hazed".to_string(),
        //     thumbnail: 0,
        //     config_file_name: "hot_tide.toml".to_string(),
        // },
    ]
}

/// 指定された曲情報ファイルから曲の情報を持ったリソースを返す.
fn load_song_config(
    filename: &str,
    speed_coeff: f32,
) -> (SongConfigResource, SongNotes, Bpm, Beat) {
    let mut file = File::open(format!("assets/songs/{}", filename)).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file into String");

    // serdeを用いてパースする
    let parsed: SongConfigParser =
        serde_yaml::from_str(&contents).expect("Couldn't parse into SongConfigParser");

    let song_config = SongConfig::from(parsed);

    let mut config_notes = song_config.notes.clone();
    // 小節線ノートを加える
    let last_bar_num = config_notes.iter().last().unwrap().bar;
    for bar in 0..(last_bar_num + 2) {
        config_notes.push(NoteSpawn {
            note_type: NoteType::BarLine,
            bar,
            beat: 0.0,
        })
    }

    // ノーツをソートする.
    config_notes.sort_by(|a, b| match a.bar.cmp(&b.bar) {
        std::cmp::Ordering::Equal => a.beat.partial_cmp(&b.beat).unwrap(),
        _ => a.bar.cmp(&b.bar),
    });

    // ノーツを配列に収める
    let bpm_resource = Bpm(song_config.initial_bpm);
    let beat_resource = Beat(song_config.initial_beat);
    #[allow(unused_mut)]
    let mut beat_par_bar = song_config.initial_beat; // 拍子
    #[allow(unused_mut)]
    let mut bpm = song_config.initial_bpm;
    // 判定線への到達タイムを蓄積させる変数
    // 途中でBPMや拍子を変更するようなイベントがあればそれを反映する.
    // 判定線に到達する時間を曲開始時刻から測ったもの.
    let mut target_time = 0.0;
    let speed = speed_coeff * BASIC_NOTE_SPEED;
    let mut notes = vec![];
    let mut prev_beat = 0.0;

    let mut prev_bar = 0u32;
    for note in config_notes {
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

    (
        song_config.into(),
        SongNotes(VecDeque::from_iter(notes)),
        bpm_resource,
        beat_resource,
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
) {
    // 型なしのアセット列を用意
    let mut assets_loading_vec = Vec::<HandleUntyped>::new();

    // 次がどのシーンに行くかによって分岐.
    match next_scene.0 {
        AppState::HomeMenu => {}
        AppState::SongSelect => {
            let assets =
                SongSelectAssetHandles::new(&asset_server, &mut texture_atlas, &mut meshes);
            // 読み込んだハンドルを型を外してクローンした配列をもらう.
            assets_loading_vec.extend(assets.to_untyped_vec());
            commands.insert_resource(assets);

            // 全曲データを読み込む
            let data = load_all_config_file_data();
            commands.insert_resource(AllSongData(
                data.into_iter().map(|data| data.into()).collect_vec(),
            ));
        }
        AppState::Game => {
            // ゲームステートに遷移する前にはこれらのリソースを用意しておかなければならない.
            let selected_song = selected_song.unwrap();
            let speed = speed.unwrap();

            // 曲データをロード
            let (config, notes, bpm, beat) =
                load_song_config(&selected_song.config_file_name, speed.0);
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
                "Now Loading...",
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
        app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_assets));
        app.add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_assets_ready));
        app.add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit_loading));
    }
}
