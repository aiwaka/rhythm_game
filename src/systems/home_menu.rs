use bevy::{app::AppExit, prelude::*};
use itertools::Itertools;

use crate::{
    components::home_menu::{ActiveOption, HomeMenuObject, HomeMenuOption, HomeMenuOptionItem},
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    resources::{
        config::NoteSpeed,
        game_state::{ExistingEntities, NextAppState},
        handles::HomeMenuAssetHandles,
    },
    AppState,
};

fn setup_node(
    mut commands: Commands,
    handles: Res<HomeMenuAssetHandles>,
    already_exist_q: Query<Entity>,
) {
    // シーン遷移時点で存在しているエンティティをすべて保存
    commands.insert_resource(ExistingEntities(already_exist_q.iter().collect_vec()));

    // 背景を出現
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
                ..Default::default()
            },
            texture: handles.background.clone(),
            ..Default::default()
        })
        .insert(HomeMenuObject);

    // 選択肢
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .insert(HomeMenuObject)
        .insert(ActiveOption(0))
        .with_children(|parent| {
            // カードを並べる
            for (idx, opt) in [HomeMenuOption::Start, HomeMenuOption::Exit]
                .iter()
                .enumerate()
            {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..Default::default()
                        },
                        background_color: Color::ANTIQUE_WHITE.into(),
                        ..Default::default()
                    })
                    .insert(*opt)
                    .insert(HomeMenuOptionItem(idx))
                    // テキスト
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("{:?}", opt),
                            TextStyle {
                                font: handles.main_font.clone(),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                        ));
                    });
            }
        });
}

/// 選択中の選択肢を強調
fn highlight_option(
    active_q: Query<&ActiveOption>,
    mut q: Query<(&HomeMenuOptionItem, &mut BackgroundColor)>,
) {
    if let Ok(active) = active_q.get_single() {
        for (card, mut color) in q.iter_mut() {
            if card.0 == active.0 {
                color.0 = Color::RED;
            } else {
                color.0 = Color::ANTIQUE_WHITE;
            }
        }
    }
}

/// 方向キーでカードを選択する
fn move_cursor(
    mut list_q: Query<(&mut ActiveOption, &mut Style, &Node, &Children)>,
    item_q: Query<(&HomeMenuOptionItem, &Node)>,
    key_input: Res<Input<KeyCode>>,
) {
    if key_input.any_just_pressed([KeyCode::Up, KeyCode::Down]) {
        let item_num = item_q.iter().len();

        if let Ok((mut active, mut style, node, children)) = list_q.get_single_mut() {
            let delta_idx = if key_input.just_pressed(KeyCode::Down) {
                1
            } else if key_input.just_pressed(KeyCode::Up) {
                // usizeは負の数を取れない.
                // あとで割った余りを結果とするので、減算は全数-1を足すことで表現する.
                item_num - 1
            } else {
                0
            };
            // リストに含まれるカードの幅の合計
            let items_height = children
                .iter()
                // 幅を読み取る
                .map(|ent| item_q.get(*ent).unwrap().1.size().y)
                .sum::<f32>();

            let list_height = node.size().y;
            // はみ出たぶんだけスクロール可能. はみ出さないなら0になる.
            let max_scroll = (list_height - items_height).max(0.0);

            // アクティブカードのインデックスを更新する
            active.0 = ((active.0 + delta_idx) % item_num).clamp(0, item_num - 1);
            style.position.top = Val::Px((-20.0 * active.0 as f32).clamp(-max_scroll, 0.0));
        }
    }
}

/// 決定キーで選択
fn determine_option(
    mut commands: Commands,
    list_q: Query<&ActiveOption>,
    card_q: Query<(&HomeMenuOptionItem, &HomeMenuOption)>,
    key_input: Res<Input<KeyCode>>,
    mut exit_ev_writer: EventWriter<AppExit>,
    mut state: ResMut<State<AppState>>,
) {
    if key_input.just_pressed(KeyCode::Z) {
        if let Ok(active) = list_q.get_single() {
            if let Some((_, opt)) = card_q.iter().find(|(card, _)| card.0 == active.0) {
                match opt {
                    HomeMenuOption::Start => {
                        // ハイスピ設定を入れる
                        commands.insert_resource(NoteSpeed(2.0));
                        commands.insert_resource(NextAppState(AppState::SongSelect));
                        state.set(AppState::Loading).unwrap();
                    }
                    HomeMenuOption::Exit => exit_ev_writer.send(AppExit),
                }
            } else {
                panic!("cannot specify the menu option.");
            }
        }
    }
}

fn despawn_home_menu_state(
    mut commands: Commands,
    already_exist: Res<ExistingEntities>,
    entity_q: Query<Entity>,
) {
    for ent in entity_q.iter() {
        // もとからあったものではないエンティティをすべて削除する
        if !already_exist.0.contains(&ent) {
            commands.entity(ent).despawn();
        }
    }
    commands.remove_resource::<ExistingEntities>();
    // 最後にアセットを破棄
    commands.remove_resource::<HomeMenuAssetHandles>();
}

pub struct HomeMenuPlugin;
impl Plugin for HomeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::HomeMenu).with_system(setup_node));
        app.add_system_set(SystemSet::on_update(AppState::HomeMenu).with_system(highlight_option));
        app.add_system_set(SystemSet::on_update(AppState::HomeMenu).with_system(move_cursor));
        app.add_system_set(SystemSet::on_update(AppState::HomeMenu).with_system(determine_option));
        app.add_system_set(
            SystemSet::on_exit(AppState::HomeMenu).with_system(despawn_home_menu_state),
        );
    }
}
