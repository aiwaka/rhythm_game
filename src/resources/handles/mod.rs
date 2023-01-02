use bevy::prelude::*;

pub mod game;
pub mod home_menu;
pub mod song_select;

pub use game::GameAssetsHandles;
pub use home_menu::HomeMenuAssetHandles;
pub use song_select::SongSelectAssetHandles;

/// アセットを読み込む際に型を考えずにロードできるようにするためのリソース.
#[derive(Resource)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

pub trait AssetHandles {
    /// 型付けされていないハンドルの列に変換する.
    /// これについてイテレートしてすべてのアセットがロード済みかどうかを確認できる.
    /// あたらしくアセットを追加した場合, 直接ファイルを読みに行くものについてのみを追加する.
    fn to_untyped_vec(&self) -> Vec<HandleUntyped>;
}
