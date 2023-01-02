use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::prelude::*;

use crate::{
    components::note::KeyLane,
    constants::{BASIC_NOTE_SPEED, LANE_WIDTH, NOTE_SPAWN_Y, TARGET_Y},
};

use super::{note::NoteType, song_list::SongData};

/// アセットを読み込む際に型を考えずにロードできるようにするためのリソース.
#[derive(Resource)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

pub trait AssetHandles {
    /// 型付けされていないハンドルの列に変換する.
    /// これについてイテレートしてすべてのアセットがロード済みかどうかを確認できる.
    /// あたらしくアセットを追加した場合, 直接ファイルを読みに行くものについてのみを追加する.
    fn to_untyped_vec(&self) -> Vec<HandleUntyped>;
}

/// 曲セレクトシーンにおけるアセットハンドル
#[derive(Debug, Resource)]
pub struct HomeMenuAssetHandles {
    // フォント
    pub main_font: Handle<Font>,

    // 画像
    pub background: Handle<Image>,
}
impl HomeMenuAssetHandles {
    pub fn new(server: &Res<AssetServer>) -> Self {
        Self {
            main_font: server.load("fonts/FiraSans-Bold.ttf"),

            background: server.load("images/backg_2.png"),
        }
    }
}
impl AssetHandles for HomeMenuAssetHandles {
    fn to_untyped_vec(&self) -> Vec<HandleUntyped> {
        let v = vec![
            self.main_font.clone_untyped(),
            self.background.clone_untyped(),
        ];
        v
    }
}

/// 曲セレクトシーンにおけるアセットハンドル
#[derive(Debug, Resource)]
pub struct SongSelectAssetHandles {
    // フォント
    pub main_font: Handle<Font>,

    // 画像
    pub background: Handle<Image>,

    /// サムネ画像メッシュ
    pub thumb_mesh: Handle<Mesh>,
    // サムネ用マテリアル
    pub thumb_img: HashMap<String, Handle<Image>>,
}

impl SongSelectAssetHandles {
    pub fn new(
        server: &Res<AssetServer>,
        _texture_atlas: &mut ResMut<Assets<TextureAtlas>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        song_data: &[SongData],
    ) -> Self {
        // let numbers = server.load("images/numbers.png");
        let thumb_shape = shape::Quad::new(Vec2::new(80.0, 80.0 * 1.6));
        let mut thumb_img = HashMap::<String, Handle<Image>>::new();
        for data in song_data {
            let img = server.load(format!("images/thumb/{}", data.thumbnail));
            thumb_img.insert(data.name.clone(), img.clone());
        }

        Self {
            main_font: server.load("fonts/FiraSans-Bold.ttf"),

            background: server.load("images/backg_2.png"),

            thumb_mesh: meshes.add(thumb_shape.into()),
            thumb_img,
        }
    }
}
impl AssetHandles for SongSelectAssetHandles {
    fn to_untyped_vec(&self) -> Vec<HandleUntyped> {
        let mut v = vec![
            self.main_font.clone_untyped(),
            self.background.clone_untyped(),
        ];
        v.extend(self.thumb_img.values().map(|img| img.clone_untyped()));
        v
    }
}

/// ゲームシーンのアセットハンドルを持っておく構造体.
#[derive(Debug, Resource)]
pub struct GameAssetsHandles {
    // フォント
    pub main_font: Handle<Font>,

    // 曲
    pub music: Handle<AudioSource>,

    // 色
    pub color_material_red: Handle<ColorMaterial>,
    pub color_material_blue: Handle<ColorMaterial>,
    pub color_material_green: Handle<ColorMaterial>,
    pub color_material_white: Handle<ColorMaterial>,
    pub color_material_white_trans: Handle<ColorMaterial>,
    pub color_material_trans: Handle<ColorMaterial>,
    // 4鍵それぞれで色を用意するとエフェクトとして使える
    pub color_material_lane_background: Vec<Handle<ColorMaterial>>,

    // メッシュ
    pub note: Handle<Mesh>,
    pub bar_note: Handle<Mesh>,
    pub judge_line: Handle<Mesh>,
    pub lane_line: Handle<Mesh>,
    pub lane_background: Handle<Mesh>,

    // atlas
    pub atlas_numbers: Handle<TextureAtlas>,

    // 一枚絵
    pub background: Handle<Image>,

    // 以下は分割画像アセットのもととなる画像アセットのハンドル. 公開はしない.
    numbers: Handle<Image>,
}

impl GameAssetsHandles {
    /// アセットをロードしてハンドルとして保持しておく
    pub fn new(
        music_filename: String,
        server: &Res<AssetServer>,
        texture_atlas: &mut ResMut<Assets<TextureAtlas>>,
        color_material: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Self {
        let numbers = server.load("images/numbers.png");
        let note_shape = shape::Quad::new(Vec2::new(100.0, 8.0));
        let bar_note_shape = shape::Quad::new(Vec2::new(400.0, 4.0));
        let judge_line_shape = shape::Quad::new(Vec2::new(700.0, 6.0));
        let lane_line_shape = shape::Quad::new(Vec2::new(4.0, 500.0));
        let lane_background_shape = shape::Quad::new(Vec2::new(LANE_WIDTH, 500.0));

        let color_material_lane_background = vec![
            color_material.add(ColorMaterial::from(Color::CRIMSON)),
            color_material.add(ColorMaterial::from(Color::SEA_GREEN)),
            color_material.add(ColorMaterial::from(Color::SEA_GREEN)),
            color_material.add(ColorMaterial::from(Color::CRIMSON)),
        ];
        Self {
            main_font: server.load("fonts/FiraSans-Bold.ttf"),

            music: server.load(&*format!("songs/{}", music_filename)),

            color_material_red: color_material.add(ColorMaterial::from(Color::RED)),
            color_material_blue: color_material.add(ColorMaterial::from(Color::BLUE)),
            color_material_green: color_material.add(ColorMaterial::from(Color::GREEN)),
            color_material_white: color_material.add(ColorMaterial::from(Color::WHITE)),
            color_material_white_trans: color_material
                .add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.5))),
            color_material_trans: color_material.add(ColorMaterial::from(Color::NONE)),
            color_material_lane_background,

            note: meshes.add(Mesh::from(note_shape)),
            bar_note: meshes.add(bar_note_shape.into()),
            judge_line: meshes.add(Mesh::from(judge_line_shape)),
            lane_line: meshes.add(Mesh::from(lane_line_shape)),
            lane_background: meshes.add(Mesh::from(lane_background_shape)),

            atlas_numbers: texture_atlas.add(TextureAtlas::from_grid(
                numbers.clone(),
                Vec2::new(30.0, 55.0),
                10,
                1,
                None,
                None,
            )),
            numbers,
            background: server.load("images/backg_2.png"),
        }
    }
    pub fn get_mesh_from_note_type(
        &self,
        note_type: &NoteType,
        speed: f32,
        bpm: f32,
        edit_mode: bool,
    ) -> ColorMesh2dBundle {
        // エディット時は下から出現するため出現位置を調整したものを用意する
        const EDIT_NOTE_SPAWN_Y: f32 = (NOTE_SPAWN_Y - TARGET_Y) * -1.0 + TARGET_Y;
        let spawn_y = if edit_mode {
            EDIT_NOTE_SPAWN_Y
        } else {
            NOTE_SPAWN_Y
        };
        match note_type {
            NoteType::Normal { key } => {
                let transform = Transform {
                    translation: Vec3::new(KeyLane::x_coord_from_num(*key), spawn_y, 1.0),
                    ..Default::default()
                };
                ColorMesh2dBundle {
                    mesh: self.note.clone().into(),
                    material: self.color_material_blue.clone(),
                    transform,
                    ..Default::default()
                }
            }
            NoteType::BarLine => {
                let transform = Transform {
                    translation: Vec3::new(0.0, spawn_y, 0.5),
                    ..Default::default()
                };
                ColorMesh2dBundle {
                    mesh: self.bar_note.clone().into(),
                    material: self.color_material_white_trans.clone(),
                    transform,
                    ..Default::default()
                }
            }
            NoteType::AdLib { key } => {
                let transform = Transform {
                    translation: Vec3::new(KeyLane::x_coord_from_num(*key), spawn_y, 1.0),
                    ..Default::default()
                };
                ColorMesh2dBundle {
                    mesh: self.note.clone().into(),
                    material: self.color_material_red.clone(),
                    transform,
                    ..Default::default()
                }
            }
            NoteType::Long { key, length } => {
                // 拍数 * 移動量(px/秒) / (拍/秒) で長さを計算
                let note_height = length * speed * BASIC_NOTE_SPEED / bpm * 60.0;
                let transform = Transform {
                    translation: Vec3::new(
                        KeyLane::x_coord_from_num(*key),
                        spawn_y - note_height / 2.0,
                        1.0,
                    ),
                    // 8.0はメッシュのy長さ.
                    scale: Vec3::new(1.0, note_height / 8.0, 1.0),
                    ..Default::default()
                };
                ColorMesh2dBundle {
                    mesh: self.note.clone().into(),
                    material: self.color_material_white_trans.clone(),
                    transform,
                    ..Default::default()
                }
            }
        }
    }
}
impl AssetHandles for GameAssetsHandles {
    fn to_untyped_vec(&self) -> Vec<HandleUntyped> {
        // let assets_loading_vec = vec![];
        vec![
            // フォント
            self.main_font.clone_untyped(),
            // 曲
            self.music.clone_untyped(),
            // 画像類
            self.numbers.clone_untyped(),
            self.background.clone_untyped(),
        ]
    }
}
