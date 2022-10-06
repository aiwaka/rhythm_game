use bevy::{asset::LoadState, prelude::*};

use crate::{
    components::{
        load::NowLoadingText,
        note::{NoteTime, SongConfig, SongConfigToml},
    },
    resources::{
        game_scene::NextAppState,
        handles::{AssetsLoading, GameAssetsHandles},
        score::ScoreResource,
    },
    AppState,
};
use std::fs::File;
use std::io::prelude::*;

fn load_config(path: &str) -> (SongConfig, String) {
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
        .map(NoteTime::new)
        .collect::<Vec<NoteTime>>();
    // 出現順にソート
    notes.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

    (
        SongConfig {
            name: parsed.name,
            notes,
        },
        parsed.filename,
    )
}

fn preload_stage_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    next_scene: Res<NextAppState>,
    mut color_material: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // 曲データをロード
    let (config, music_filename) = load_config("test.toml");
    commands.insert_resource(config);

    // 型なしのアセット列を用意
    let mut assets_loading_vec = Vec::<HandleUntyped>::new();

    // 次がどのシーンに行くかによって分岐.
    match next_scene.0 {
        AppState::HomeMenu => {}
        AppState::Game => {
            info!("arrange handles");
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
        app.add_system_set(
            SystemSet::on_enter(AppState::Loading).with_system(preload_stage_assets),
        );
        app.add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_assets_ready));
        app.add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit_loading));
    }
}
