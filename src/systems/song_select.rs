use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    components::{
        song_select::{ActiveSongCard, SongData, SongSelectCard},
        timer::FrameCounter,
    },
    resources::{
        game_scene::{ExistingEntities, NextAppState},
        handles::SongSelectAssetHandles,
        song::{SelectedSong, Speed},
        song_list::AllSongData,
    },
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
    // let song_num = all_song_data.0.len();

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
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
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
                            ..Default::default()
                        },
                        background_color: Color::ANTIQUE_WHITE.into(),
                        ..Default::default()
                    })
                    .insert(FrameCounter::new())
                    .insert(SongSelectCard(idx))
                    // 曲データをくっつけておく
                    .insert(song_data.clone())
                    .with_children(|parent| {
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
                color.0 = Color::rgb(0.7, 0.8, 0.8 + 0.1 * param);
            } else {
                color.0 = Color::ANTIQUE_WHITE;
            }
        }
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

            active.0 = ((active.0 + delta_idx) % item_num).clamp(0, item_num - 1);
            style.position.left =
                Val::Px((-(CARD_WIDTH + 20.0) * active.0 as f32).clamp(-max_scroll, 0.0));
        }
    }
}

/// 決定キーで曲を選択
fn determine_song(
    mut commands: Commands,
    list_q: Query<&ActiveSongCard>,
    card_q: Query<(&SongSelectCard, &SongData)>,
    key_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if key_input.just_pressed(KeyCode::Z) {
        if let Ok(active) = list_q.get_single() {
            if let Some((_, song_data)) = card_q.iter().find(|(card, _)| card.0 == active.0) {
                info!("select song {:?}", song_data);
                // 必要な情報をセットしてからステート移行
                commands.insert_resource(SelectedSong::from_song_card(song_data));
                commands.insert_resource(Speed(1.5));
                commands.insert_resource(NextAppState(AppState::Game));
                state.set(AppState::Loading).unwrap();
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

pub struct SongSelectStatePlugin;
impl Plugin for SongSelectStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::SongSelect).with_system(setup_song_select_scene),
        );
        app.add_system_set(SystemSet::on_update(AppState::SongSelect).with_system(hover_card));
        app.add_system_set(SystemSet::on_update(AppState::SongSelect).with_system(move_cursor));
        app.add_system_set(SystemSet::on_update(AppState::SongSelect).with_system(determine_song));
        app.add_system_set(
            SystemSet::on_exit(AppState::SongSelect).with_system(despawn_song_select_scene),
        );
    }
}
