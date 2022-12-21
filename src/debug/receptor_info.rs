use bevy::prelude::*;

use crate::components::receptor::{prelude::*, PatternReceptor, PatternReceptorMarker};

#[derive(Component)]
struct DebugWindow {
    pub item_num: usize,
}
/// レセプタリストのアイテムを表すノード. 番号を持ち区別する.
#[derive(Component)]
struct DebugListNode(pub usize);
/// ウィンドウの直下に配置される情報テキストノード
#[derive(Component)]
struct DebugInfoTextNode;

/// ウィンドウ表示時にレセプタに付与する.
/// 番号を持ち, ターゲットとして表示したいオブジェクトを探すための識別子となる.
#[derive(Component)]
struct DebugObject(pub usize);

/// デバッグウィンドウにくっつける. どのオブジェクト番号を指すかを指定する.
#[derive(Component)]
struct TargetObject(pub usize);

/// デバッグウィンドウや関連エンティティを消去する
fn hide_receptor_list(
    mut commands: Commands,
    window_q: Query<Entity, With<DebugWindow>>,
    obj_q: Query<Entity, With<DebugObject>>,
    key_input: Res<Input<KeyCode>>,
) {
    if window_q.is_empty() {
        return;
    }
    if !key_input.just_pressed(KeyCode::O) {
        return;
    }
    // デバッグウィンドウは再帰的に消去
    let ent = window_q.get_single().unwrap();
    commands.entity(ent).despawn_recursive();
    // デバッグ情報を付与していたオブジェクト（レセプタ）に対してはコンポーネントを取り除く
    for ent in obj_q.iter() {
        commands.entity(ent).remove::<DebugObject>();
    }
}

/// オブジェクト一覧を表示するデバッグウィンドウを出現させる
fn show_receptor_list(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    receptor_q: Query<(&PatternReceptorMarker, Entity)>,
    key_input: Res<Input<KeyCode>>,
    window_q: Query<With<DebugWindow>>,
) {
    if !window_q.is_empty() {
        return;
    }
    if !key_input.just_pressed(KeyCode::O) {
        return;
    }

    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let window_ent = commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Column,
                max_size: Size::new(Val::Undefined, Val::Percent(20.0)),
                overflow: Overflow::Hidden,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 0.4)),
            ..Default::default()
        })
        .insert(DebugWindow {
            item_num: receptor_q.iter().len(),
        })
        .insert(TargetObject(0))
        .id();
    // 情報テキストノード
    let text_node_ent = commands
        .spawn(NodeBundle {
            background_color: Color::WHITE.into(),
            ..Default::default()
        })
        .insert(DebugInfoTextNode)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::DARK_GREEN,
                },
            ));
        })
        .id();
    // リストを包むノード
    let list_node = commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 0.7)),
            ..Default::default()
        })
        .id();
    for (idx, (receptor_marker, receptor_ent)) in receptor_q.iter().enumerate() {
        // レセプタに対応するリスト要素ノード
        let node_ent = commands
            .spawn(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            })
            .insert(DebugListNode(idx))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    receptor_marker.0.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::GRAY,
                    },
                ));
            })
            .id();
        // リストにテキストノードを追加し, 対応する番号を表すコンポーネントを各オブジェクトに追加する
        commands.entity(list_node).add_child(node_ent);
        commands.entity(receptor_ent).insert(DebugObject(idx));
    }
    // リストノードと情報表示ノードをウィンドウに追加
    commands.entity(window_ent).add_child(list_node);
    commands.entity(window_ent).add_child(text_node_ent);
}

/// 横キーでtargetの番号を変更する.
fn move_cursor(mut q: Query<(&mut TargetObject, &DebugWindow)>, key_input: Res<Input<KeyCode>>) {
    if let Ok((mut target, window)) = q.get_single_mut() {
        if key_input.just_pressed(KeyCode::Right) {
            target.0 += 1;
            if target.0 == window.item_num {
                target.0 = 0;
            }
        }
    }
}

/// ターゲット番号と描画の対応付を行う.
fn list_cursor(
    node_q: Query<(&DebugListNode, &Children)>,
    mut text_q: Query<&mut Text>,
    target_q: Query<&TargetObject>,
) {
    // ターゲット番号を取得
    if let Ok(target) = target_q.get_single() {
        // info!("{}", target.0);
        // ノードとその子コンポーネントを取得
        for (node, children) in node_q.iter() {
            for &child in children.iter() {
                // テキストの色を変更
                if let Ok(mut text) = text_q.get_mut(child) {
                    text.sections[0].style.color = if node.0 == target.0 {
                        Color::RED
                    } else {
                        Color::default()
                    }
                }
            }
        }
    }
}

/// 注目しているレセプタの情報を見る
fn show_info<T: PatternReceptor>(
    window_q: Query<With<DebugWindow>>,
    info_q: Query<&Children, With<DebugInfoTextNode>>,
    mut text_q: Query<&mut Text>,
    target_q: Query<&TargetObject>,
    receptor_q: Query<(&T, &DebugObject)>,
) {
    if window_q.is_empty() {
        return;
    }

    if let Ok(target_obj) = target_q.get_single() {
        if let Some((receptor, _)) = receptor_q.iter().find(|(_, obj)| obj.0 == target_obj.0) {
            if let Ok(children) = info_q.get_single() {
                for &child in children.iter() {
                    if let Ok(mut text) = text_q.get_mut(child) {
                        text.sections[0].value = receptor.debug_display();
                    }
                }
            }
        }
    }
}

pub(super) struct ReceptorInfoPlugin;
impl Plugin for ReceptorInfoPlugin {
    fn build(&self, app: &mut App) {
        /// レセプタ構造体をappに追加するマクロ.
        macro_rules! add_receptor_to_system {
            ($receptor:ty) => {
                app.add_system(show_info::<$receptor>)
            };
        }
        app.add_system(show_receptor_list);
        app.add_system(hide_receptor_list);
        app.add_system(move_cursor);
        app.add_system(list_cursor);

        add_receptor_to_system!(FullSyncReceptor);
        add_receptor_to_system!(StepRightReceptor);
        add_receptor_to_system!(StepLeftReceptor);
        add_receptor_to_system!(DoubleTapReceptor);
        add_receptor_to_system!(TrillReceptor);
    }
}
