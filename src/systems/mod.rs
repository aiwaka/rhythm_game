pub mod audio;
pub mod editor;
pub mod home_menu;
pub mod load;
pub mod note;
pub mod receptor;
pub mod result_screen;
pub mod score;
pub mod song_select;
mod system_labels;
pub mod timer;
pub mod ui;

/// (app, state, system, \[(after | before) : label, ...] (optional), label (optional))
///
/// という形式で, 毎フレーム実行されるシステムをappに登録する.
/// ステートは`AppState`の列挙子のいずれかを指定する（`AppState::`は不要）.
/// システムにlabelを付与したいが`after`等は必要ない場合`[]`のみ書く必要がある.
#[macro_export]
macro_rules! add_update_system {
    ($app: expr, $state: ident, $system: expr $(, [$($after_before: ident : $label: expr),+] $(, $system_label: expr)?)?) => {
        $app.add_system_set(
            SystemSet::on_update(AppState::$state)
                .with_system($system)
                $(
                    $(.$after_before($label))+
                    $(.label($system_label))?
                )?
            )
    };
}
/// (app, state, system, \[(after | before) : label, ...] (optional), label (optional))
///
/// という形式で, on_enterで実行されるシステムをappに登録する.
/// ステートは`AppState`の列挙子のいずれかを指定する（`AppState::`は不要）.
/// システムにlabelを付与したいが`after`等は必要ない場合`[]`のみ書く必要がある.
#[macro_export]
macro_rules! add_enter_system {
    ($app: expr, $state: ident, $system: expr $(, [$($after_before: ident : $label: expr),+] $(, $system_label: expr)?)?) => {
        $app.add_system_set(
            SystemSet::on_enter(AppState::$state)
                .with_system($system)
                $(
                    $(.$after_before($label))+
                    $(.label($system_label))?
                )?
            )
    };
}
/// (app, state, system, \[(after | before) : label, ...] (optional), label (optional))
///
/// という形式で, on_exitで実行されるシステムをappに登録する.
/// ステートは`AppState`の列挙子のいずれかを指定する（`AppState::`は不要）.
/// システムにlabelを付与したいが`after`等は必要ない場合`[]`のみ書く必要がある.
#[macro_export]
macro_rules! add_exit_system {
    ($app: expr, $state: ident, $system: expr $(, [$($after_before: ident : $label: expr),+] $(, $system_label: expr)?)?) => {
        $app.add_system_set(
            SystemSet::on_exit(AppState::$state)
                .with_system($system)
                $(
                    $(.$after_before($label))+
                    $(.label($system_label))?
                )?
            )
    };
}
