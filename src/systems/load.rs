use bevy::{asset::LoadState, prelude::*};
use itertools::Itertools;

use crate::{
    components::{load::NowLoadingText, note::Note, song_select::SongData},
    resources::{
        game_scene::NextAppState,
        handles::{AssetHandles, AssetsLoading, GameAssetsHandles, SongSelectAssetHandles},
        score::ScoreResource,
        song::{SelectedSong, Speed},
        song::{SongConfig, SongConfigToml},
        song_list::{AllSongData, SongDataToml},
    },
    AppState,
};
use std::io::prelude::*;
use std::{collections::VecDeque, fs::File};

/// 曲一覧情報を取得する.
/// TODO: 現在ハードコーディングしているが, tomlファイルから読み込むように変更する.
fn load_all_config_file_data() -> Vec<SongDataToml> {
    vec![
        SongDataToml {
            name: "test1".to_string(),
            thumbnail: 0,
            config_file_name: "test.toml".to_string(),
        },
        SongDataToml {
            name: "test2".to_string(),
            thumbnail: 0,
            config_file_name: "test.toml".to_string(),
        },
        SongDataToml {
            name: "test3".to_string(),
            thumbnail: 0,
            config_file_name: "test.toml".to_string(),
        },
        SongDataToml {
            name: "test4".to_string(),
            thumbnail: 0,
            config_file_name: "test.toml".to_string(),
        },
    ]
}

/// 指定された曲情報ファイルから曲の情報を持ったリソースを返す.
fn load_config_from_toml(path: &str, speed_coeff: f32) -> SongConfig {
    let mut file = File::open(format!("assets/songs/{}", path)).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file into String");

    // toml, Serdeクレートを用いてパースする
    let parsed: SongConfigToml =
        toml::from_str(&contents).expect("Couldn't parse into SongConfigToml");

    // ノーツをパースして配列に収める
    let mut notes = parsed
        .notes
        .iter()
        .map(|note| Note::new(note, parsed.beat_par_bar, parsed.bpm, speed_coeff))
        .collect_vec();
    // 出現順にソート
    notes.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

    SongConfig {
        name: parsed.name,
        music_filename: parsed.filename,
        length: parsed.length,
        beat_par_bar: parsed.beat_par_bar,
        bpm: parsed.bpm,
        notes: VecDeque::from_iter(notes),
    }
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
    selected_song: Option<Res<SelectedSong>>,
    speed: Option<Res<Speed>>,
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
            commands.insert_resource(AllSongData(data.iter().map(SongData::new).collect_vec()));
        }
        AppState::Game => {
            // ゲームステートに遷移する前にはこれらのリソースを用意しておかなければならない.
            let selected_song = selected_song.unwrap();
            let speed = speed.unwrap();

            // 曲データをロード
            let config = load_config_from_toml(&selected_song.filename, speed.0);
            let music_filename = config.music_filename.clone();
            commands.insert_resource(config);
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
        }
        _ => {}
    }
    // ローディング中の型無しアセットとしてリソースに追加
    commands.insert_resource(AssetsLoading(assets_loading_vec));
    // ローディング中テキストエンティティを出現させる.
    commands
        .spawn_bundle(TextBundle {
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
        app.init_resource::<ScoreResource>();
        // アセットロード関連システム
        app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_assets));
        app.add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_assets_ready));
        app.add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit_loading));
    }
}
