use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    components::home_menu::{ActiveOption, HomeMenuObject, HomeMenuOption, HomeMenuOptionItem},
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    resources::{game_state::ExistingEntities, handles::HomeMenuAssetHandles},
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
                // margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::GREEN),
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

pub struct HomeMenuPlugin;
impl Plugin for HomeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::HomeMenu).with_system(setup_node));
    }
}
