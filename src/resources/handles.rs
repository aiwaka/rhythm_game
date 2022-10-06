use bevy::prelude::*;

/// アセットを読み込む際に型を考えずにロードできるようにするためのリソース.
pub struct AssetsLoading(pub Vec<HandleUntyped>);

/// ゲームシーンのアセットハンドルを持っておく構造体.
#[derive(Debug)]
pub struct GameAssetsHandles {
    pub main_font: Handle<Font>,

    pub music: Handle<AudioSource>,

    pub color_material_red: Handle<ColorMaterial>,
    pub color_material_blue: Handle<ColorMaterial>,
    pub color_material_green: Handle<ColorMaterial>,
    pub color_material_white_trans: Handle<ColorMaterial>,
    pub note: Handle<Mesh>,
    pub judge_line: Handle<Mesh>,
    pub atlas_numbers: Handle<TextureAtlas>,
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
        let judge_line_shape = shape::Quad::new(Vec2::new(400.0, 8.0));
        Self {
            main_font: server.load("fonts/FiraSans-Bold.ttf"),

            music: server.load(&*format!("songs/{}", music_filename)),
            note: meshes.add(Mesh::from(note_shape)),
            judge_line: meshes.add(Mesh::from(judge_line_shape)),
            color_material_red: color_material.add(ColorMaterial::from(Color::RED)),
            color_material_blue: color_material.add(ColorMaterial::from(Color::BLUE)),
            color_material_green: color_material.add(ColorMaterial::from(Color::GREEN)),
            color_material_white_trans: color_material
                .add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.5))),
            atlas_numbers: texture_atlas.add(TextureAtlas::from_grid(
                numbers.clone(),
                Vec2::new(30.0, 55.0),
                10,
                1,
            )),
            numbers,
            background: server.load("images/backg_2.png"),
        }
    }

    /// 型付けされていないハンドルの列に変換する.
    /// これについてイテレートしてすべてのアセットがロード済みかどうかを確認できる.
    /// あたらしくアセットを追加した場合, 直接ファイルを読みに行くものについてのみここに追加する.
    pub fn to_untyped_vec(&self) -> Vec<HandleUntyped> {
        let mut assets_loading_vec = vec![
            // フォントをロード
            self.main_font.clone_untyped(),
            // 曲
            self.music.clone_untyped(),
            // 画像類
            self.numbers.clone_untyped(),
            self.background.clone_untyped(),
        ];
        assets_loading_vec
    }
}
