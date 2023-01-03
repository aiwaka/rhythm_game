use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    components::{
        editor::FrozenChartErrorText,
        song_select::{
            ActiveSongCard, DifficultyText, SongSelectCard, SongSelectParentNode, SpeedSettingNode,
        },
        timer::FrameCounter,
    },
    resources::{
        config::{GameDifficulty, NoteSpeed},
        game_state::{ExistingEntities, NextAppState},
        handles::SongSelectAssetHandles,
        song_list::{AllSongData, SongData},
    },
    spawn_text_node,
    systems::system_labels::TimerSystemLabel,
    AppState, SCREEN_HEIGHT, SCREEN_WIDTH,
};

const CARD_WIDTH: f32 = 200.0;

fn setup_song_select_scene(
    mut commands: Commands,
    already_exist_q: Query<Entity>,
    handles: Res<SongSelectAssetHandles>,
    all_song_data: Res<AllSongData>,
) {
    // シーン遷移時点で存在しているエンティティをすべて保存
    commands.insert_resource(ExistingEntities(already_exist_q.iter().collect_vec()));
    // 背景を出現
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
            ..Default::default()
        },
        texture: handles.background.clone(),
        ..Default::default()
    });

    // 曲カードを出現
    commands
        .spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                max_size: Size::new(Val::Undefined, Val::Percent(80.0)),
                // overflow: Overflow::Hidden,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::GREEN),
            ..Default::default()
        })
        .insert(SongSelectParentNode)
        .insert(ActiveSongCard(0))
        .with_children(|parent| {
            // カードを並べる
            for (idx, song_data) in all_song_data.0.iter().enumerate() {
                // 第一項：画面の真ん中.
                // 第二項：song_numを超えない最大の奇数とカード高さの半分の積.
                // 第三項：曲のインデックスをsong_numの半分ずらして0から振り直し, 順番に配置.
                // let pos_x = 0.0 - (((song_num - 1) / 2) * 2 + 1) as f32 * CARD_WIDTH / 2.0
                //     + ((idx + song_num / 2) % song_num) as f32 * CARD_WIDTH;
                // let pos_y = 140.0;
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(CARD_WIDTH), Val::Px(CARD_WIDTH * 1.618)),
                            margin: UiRect::all(Val::Px(20.0)),
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        background_color: Color::ANTIQUE_WHITE.into(),
                        ..Default::default()
                    })
                    .insert(FrameCounter::new())
                    .insert(SongSelectCard(idx))
                    // 曲データをくっつけておく
                    .insert(song_data.clone())
                    // カードの中身のサムネイルとテキスト等
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Percent(80.0), Val::Percent(50.0)),
                                ..default()
                            },
                            image: handles
                                .thumb_img
                                .get(&song_data.name)
                                .unwrap()
                                .clone()
                                .into(),
                            ..default()
                        });
                        parent.spawn(TextBundle::from_section(
                            song_data.name.clone(),
                            TextStyle {
                                font: handles.main_font.clone(),
                                font_size: 30.0,
                                color: Color::GRAY,
                            },
                        ));
                    });
            }
        });

    // 難易度テキスト
    spawn_text_node!(commands, handles.main_font, [right: 10.0, bottom: 20.0], Color::ANTIQUE_WHITE, [["", 30.0, Color::GRAY, [DifficultyText]]], [], {size: Size::new(Val::Px(90.0), Val::Px(40.0))});
}

/// Xキーでホームに戻る
fn back_to_home_menu(
    mut commands: Commands,
    mut key_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if key_input.just_pressed(KeyCode::X) {
        key_input.reset_all();
        commands.insert_resource(NextAppState(AppState::HomeMenu));
        state.set(AppState::Loading).unwrap();
    }
}

/// 選択中のカードをふわふわさせる
fn hover_card(
    active_q: Query<&ActiveSongCard>,
    mut q: Query<(&SongSelectCard, &mut BackgroundColor, &FrameCounter)>,
) {
    if let Ok(active) = active_q.get_single() {
        for (card, mut color, counter) in q.iter_mut() {
            if card.0 == active.0 {
                let param = (counter.count() as f32 / 20.0).sin();
                color.0 = Color::rgb(0.8, 0.8, 0.8 + 0.1 * param);
            } else {
                color.0 = Color::ANTIQUE_WHITE;
            }
        }
    }
}

/// Dキーで難易度変更
fn change_difficulty(key_input: Res<Input<KeyCode>>, mut diff: ResMut<GameDifficulty>) {
    if key_input.just_pressed(KeyCode::D) {
        *diff = match *diff {
            GameDifficulty::Normal => GameDifficulty::Expert,
            GameDifficulty::Expert => GameDifficulty::Master,
            GameDifficulty::Master => GameDifficulty::Normal,
        }
    }
}

/// 難易度に応じて背景色やテキストを変化させる
fn reflect_difficulty(
    diff: Res<GameDifficulty>,
    mut node_q: Query<&mut BackgroundColor, With<SongSelectParentNode>>,
    mut text_q: Query<&mut Text, With<DifficultyText>>,
) {
    if let Ok(mut color) = node_q.get_single_mut() {
        color.0 = diff.get_color();
    }
    if let Ok(mut text) = text_q.get_single_mut() {
        text.sections[0].value = diff.to_string();
    }
}

/// 方向キーでカードを選択する
fn move_cursor(
    mut list_q: Query<(&mut ActiveSongCard, &mut Style, &Node, &Children)>,
    card_q: Query<(&SongSelectCard, &Node)>,
    key_input: Res<Input<KeyCode>>,
) {
    if key_input.any_just_pressed([KeyCode::Left, KeyCode::Right]) {
        let item_num = card_q.iter().len();

        if let Ok((mut active, mut style, node, children)) = list_q.get_single_mut() {
            let delta_idx = if key_input.just_pressed(KeyCode::Right) {
                1
            } else if key_input.just_pressed(KeyCode::Left) {
                // usizeは負の数を取れない.
                // あとで割った余りを結果とするので、減算は全数-1を足すことで表現する.
                item_num - 1
            } else {
                0
            };
            // リストに含まれるカードの幅の合計
            let items_width = children
                .iter()
                // 幅を読み取る
                .map(|ent| card_q.get(*ent).unwrap().1.size().x)
                .sum::<f32>();

            let list_width = node.size().x;
            // はみ出たぶんだけスクロール可能. はみ出さないなら0になる.
            let max_scroll = (list_width - items_width).max(0.0);

            // アクティブカードのインデックスを更新する
            active.0 = ((active.0 + delta_idx) % item_num).clamp(0, item_num - 1);
            style.position.left =
                Val::Px((-(CARD_WIDTH + 20.0) * active.0 as f32).clamp(-max_scroll, 0.0));
        }
    }
}

/// 時間経過で消去
fn update_frozen_edit_alert(
    mut commands: Commands,
    q: Query<(&FrameCounter, Entity), With<FrozenChartErrorText>>,
) {
    for (counter, ent) in q.iter() {
        if counter.count() > 60 {
            commands.entity(ent).despawn_recursive();
        }
    }
}

/// 決定キーで曲を選択
fn determine_song(
    mut commands: Commands,
    list_q: Query<&ActiveSongCard>,
    card_q: Query<(&SongSelectCard, &SongData)>,
    handles: Res<SongSelectAssetHandles>,
    key_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if key_input.just_pressed(KeyCode::Z) {
        if let Ok(active) = list_q.get_single() {
            if let Some((_, song_data)) = card_q.iter().find(|(card, _)| card.0 == active.0) {
                info!("select song {:?}", song_data);
                // 必要な情報をセットしてからステート移行
                commands.insert_resource(song_data.clone());
                // Eキーを押した状態だったら行き先をエディットモードに変更
                if key_input.pressed(KeyCode::E) {
                    if song_data.edit_freeze {
                        info!("edit freeze");
                        spawn_text_node!(
                            commands,
                            handles.main_font,
                            [left: 20.0, bottom: 20.0],
                            Color::ANTIQUE_WHITE,
                            [
                                [format!("Cannot edit this chart '{}'", song_data.name), 30.0, Color::RED, []]
                            ],
                            [FrameCounter::new(), FrozenChartErrorText],
                            { size: Size::new(Val::Auto, Val::Px(40.0)) }
                        );
                    } else {
                        commands.insert_resource(NextAppState(AppState::Editor));
                        state.set(AppState::Loading).unwrap();
                    }
                } else {
                    commands.insert_resource(NextAppState(AppState::Game));
                    state.set(AppState::Loading).unwrap();
                }
            } else {
                panic!("cannot specify the selected song.");
            }
        }
    }
}

fn despawn_song_select_scene(
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
    commands.remove_resource::<SongSelectAssetHandles>();
}

fn speed_setting_node(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    node_q: Query<(&Children, Entity), With<SpeedSettingNode>>,
    mut text_q: Query<&mut Text>,
    handles: Res<SongSelectAssetHandles>,
    mut speed_coeff: ResMut<NoteSpeed>,
) {
    if key_input.pressed(KeyCode::S) {
        if node_q.is_empty() {
            // Sが押されていてノードが表示されていないなら出す
            spawn_text_node!(
                commands,
                handles.main_font,
                [left: 20.0, bottom: 20.0],
                Color::ANTIQUE_WHITE,
                [["", 30.0, Color::BLUE, []]],
                [SpeedSettingNode],
                {size: Size::new(Val::Auto, Val::Px(30.0))}
            );
        } else {
            // 表示されているなら更新する
            let speed_int = (**speed_coeff * 10.0) as i32;
            let delta = if key_input.just_pressed(KeyCode::Up) {
                1
            } else if key_input.just_pressed(KeyCode::Down) {
                -1
            } else {
                0
            };
            **speed_coeff = ((speed_int + delta) as f32 / 10.0).clamp(0.5, 4.0);
            let (children, _) = node_q.get_single().unwrap();
            for &child in children.iter() {
                if let Ok(mut text) = text_q.get_mut(child) {
                    text.sections[0].value = format!("SPEED: {}", **speed_coeff);
                }
            }
        }
    } else {
        for (_, ent) in node_q.iter() {
            commands.entity(ent).despawn_recursive();
        }
    }
}

pub struct SongSelectStatePlugin;
impl Plugin for SongSelectStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::SongSelect).with_system(setup_song_select_scene),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::SongSelect).with_system(back_to_home_menu),
        );
        app.add_system_set(SystemSet::on_update(AppState::SongSelect).with_system(hover_card));
        app.add_system_set(
            SystemSet::on_update(AppState::SongSelect).with_system(change_difficulty),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::SongSelect).with_system(reflect_difficulty),
        );
        app.add_system_set(SystemSet::on_update(AppState::SongSelect).with_system(move_cursor));
        app.add_system_set(
            SystemSet::on_update(AppState::SongSelect)
                .with_system(update_frozen_edit_alert.after(TimerSystemLabel::FrameCounterUpdate)),
        );
        app.add_system_set(SystemSet::on_update(AppState::SongSelect).with_system(determine_song));
        app.add_system_set(
            SystemSet::on_exit(AppState::SongSelect).with_system(despawn_song_select_scene),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::SongSelect).with_system(speed_setting_node),
        );
    }
}
