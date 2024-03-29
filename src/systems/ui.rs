use bevy::{prelude::*, sprite::Mesh2dHandle};
use itertools::Itertools;
use rand::Rng;

use crate::{
    add_enter_system, add_update_system,
    components::{
        note::KeyLane,
        timer::{CountDownTimer, FrameCounter},
        ui::{
            CatchEvalPopupText, ChartInfoNode, GameStateObject, LaneLine, PatternPopupText,
            ScoreText, TargetLine,
        },
    },
    constants::{LANE_WIDTH, TARGET_Y},
    events::{AchievePatternEvent, NoteEvalEvent},
    resources::{
        config::GameDifficulty,
        game_state::ExistingEntities,
        handles::GameAssetsHandles,
        note::NoteType,
        score::{CatchEval, ScoreResource, TimingEval},
        song::SongConfigResource,
    },
    AppState, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::system_labels::{PatternReceptorSystemLabel, TimerSystemLabel, UiSystemLabel};

/// (commands, font, \[left: px, top: px], 背景色, \[\[テキスト, フォントサイズ, 色, \[テキストにくっつけるコンポーネント, ... ]], ... ], \[ノードにくっつけるコンポーネント, ... ], { ノードのStyleの設定(optional) })
///
/// という構文で, テキストノードを出現させる. left, topはright, bottomでもいい（実は順番も逆でも良い）.
/// ノードのスタイルについて, `position_type`（`Absolute`）, `position`（3番目の位置の項目で設定）, `flex_direction`（`Column`）は設定済み.
/// その他のフィールドについては最後のoptional項目で設定できる.
/// 返り値としてEntityを利用できる.
#[macro_export]
macro_rules! spawn_text_node {
    ($commands: expr, $font: expr, [ $horizontal_spec: ident : $px_x: expr, $vertical_spec: ident : $px_y: expr], $bg_color: expr, [$([$text: expr, $font_size: expr, $color: expr, [$($component: expr),*]]),+], [$($node_component: expr),*] $(, {$($style_field: ident : $style_val: expr),+})?) => {
        let font: Handle<Font> = $font.clone();
        {
            let ent = $commands
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            $horizontal_spec: Val::Px($px_x),
                            $vertical_spec: Val::Px($px_y),
                            ..Default::default()
                        },
                        flex_direction: FlexDirection::Column,
                        $($($style_field: $style_val,)+)?
                        ..Default::default()
                    },
                    background_color: BackgroundColor($bg_color),
                    ..Default::default()
                })
                .with_children(|parent| {
                    $(
                        let bundle = ($($component),*);
                        parent.spawn(TextBundle::from_section(
                            $text,
                            TextStyle {
                                font: font.clone(),
                                font_size: $font_size,
                                color: $color,
                            },
                        ))
                        .insert(bundle);
                    )+
                }).id();
                let bundle = ($($node_component),*);
                $commands.entity(ent).insert(bundle);
            ent
        }
    };
}

fn setup_ui(
    mut commands: Commands,
    song_config: Res<SongConfigResource>,
    diff: Res<GameDifficulty>,
    handles: Res<GameAssetsHandles>,
) {
    let font = handles.main_font.clone();
    // 曲名・難易度表示ノード
    spawn_text_node!(
        commands,
        font,
        [left : 10.0, top : 10.0],
        Color::NONE,
        [
            [song_config.name.clone(), 30.0, Color::WHITE, []],
            [diff.to_string(), 20.0, diff.get_color(), []]
        ],
        [GameStateObject, ChartInfoNode]
    );

    // スコア表示テキストノード
    spawn_text_node!(
        commands,
        font,
        [left: 10.0, bottom: 10.0],
        Color::NONE,
        [[
            "Score: 0. Corrects: 0. Fails: 0",
            40.0,
            Color::WHITE,
            [ScoreText]
        ]],
        [GameStateObject]
    );
    // NOTE: マクロの展開は以下のようになることを示すためここは残しておく.
    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             position_type: PositionType::Absolute,
    //             position: UiRect {
    //                 left: Val::Px(10.),
    //                 bottom: Val::Px(10.),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         },
    //         background_color: BackgroundColor(Color::NONE),
    //         ..Default::default()
    //     })
    //     .insert(GameStateObject)
    //     .with_children(|parent| {
    //         parent
    //             .spawn(TextBundle {
    //                 text: Text {
    //                     sections: vec![TextSection {
    //                         value: "Score: 0. Corrects: 0. Fails: 0".to_string(),
    //                         style: TextStyle {
    //                             font: font.clone(),
    //                             font_size: 40.0,
    //                             color: Color::rgb(0.8, 0.8, 0.8),
    //                         },
    //                     }],
    //                     ..Default::default()
    //                 },
    //                 ..Default::default()
    //             })
    //             .insert(ScoreText);
    //     });

    // 判定線
    let transform = Transform {
        translation: Vec3::new(0.0, TARGET_Y, 2.0),
        ..Default::default()
    };
    commands
        .spawn(ColorMesh2dBundle {
            mesh: Mesh2dHandle::from(handles.judge_line.clone()),
            material: handles.color_material_white.clone(),
            transform,
            ..Default::default()
        })
        .insert(TargetLine)
        .insert(GameStateObject);

    // 鍵盤線
    for i in 0..5 {
        let x = KeyLane::x_coord_from_num(i);
        let transform = Transform {
            translation: Vec3::new(x - LANE_WIDTH / 2.0, TARGET_Y + 250.0, 2.0),
            ..Default::default()
        };
        commands
            .spawn(ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(handles.lane_line.clone()),
                material: handles.color_material_white.clone(),
                transform,
                ..Default::default()
            })
            .insert(LaneLine)
            .insert(GameStateObject);
    }
}

fn setup_lane(
    mut commands: Commands,
    handles: Res<GameAssetsHandles>,
    already_exist_q: Query<Entity>,
) {
    // シーン遷移時点で存在しているエンティティをすべて保存
    commands.insert_resource(ExistingEntities(already_exist_q.iter().collect_vec()));
    for i in 0..4 {
        let x = KeyLane::x_coord_from_num(i);
        let transform = Transform {
            translation: Vec3::new(x, TARGET_Y + 250.0, 0.1),
            ..Default::default()
        };
        commands
            .spawn(ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(handles.lane_background.clone()),
                material: handles.color_material_lane_background[i as usize].clone(),
                transform,
                ..Default::default()
            })
            .insert(KeyLane(i))
            .insert(GameStateObject)
            .insert(FrameCounter::new_default(60));
    }
}

// fn update_time_text(
//     mut query: Query<(&mut Text, &TimeText)>,
//     start_time: Res<SongStartTime>,
//     time: Res<Time>,
// ) {
//     let time_after_start = start_time.time_after_start(&time);
//     if time_after_start < 0.0 {
//         return;
//     }
//     for (mut text, _marker) in query.iter_mut() {
//         text.sections[0].value = format!("Time: {:.2}", time_after_start);
//     }
// }

fn update_score_text(score: Res<ScoreResource>, mut query: Query<(&mut Text, &ScoreText)>) {
    if score.is_changed() {
        for (mut text, _marker) in query.iter_mut() {
            text.sections[0].value = format!(
                "Score: {}. Perfect: {}. Ok: {}. Miss: {}.",
                score.get_score(),
                score.get_eval_num(&CatchEval::Perfect)
                    + score.get_eval_num(&CatchEval::NearPerfect(TimingEval::Fast))
                    + score.get_eval_num(&CatchEval::NearPerfect(TimingEval::Slow)),
                score.get_eval_num(&CatchEval::Ok(TimingEval::Fast))
                    + score.get_eval_num(&CatchEval::Ok(TimingEval::Slow)),
                score.get_eval_num(&CatchEval::Miss)
            );
        }
    }
}

fn update_lane_background(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Handle<ColorMaterial>, &KeyLane, &mut FrameCounter)>,
    key_input: Res<Input<KeyCode>>,
) {
    for (color, lane, mut counter) in query.iter_mut() {
        if lane.key_just_pressed(&key_input) {
            counter.reset();
        }
        let opacity = 1.0 - counter.count().clamp(0, 10) as f32 / 10.0;
        let new_color = &mut materials.get_mut(color).unwrap().color;
        new_color.set_a(opacity);
    }
}

fn spawn_pattern_text(
    mut commands: Commands,
    mut ev_reader: EventReader<AchievePatternEvent>,
    handles: Res<GameAssetsHandles>,
) {
    // 乱数生成器
    let mut rng = rand::thread_rng();

    let font = handles.main_font.clone();
    for ev in ev_reader.iter() {
        // 出現位置をある程度ランダムに
        #[allow(clippy::if_same_then_else)]
        let pos_x: f32 = if rng.gen_bool(0.5) {
            rng.gen_range(10.0..=40.0)
        } else {
            rng.gen_range(580.0..=620.0)
        };
        let pos_y: f32 = rng.gen_range(200.0..=300.0);
        spawn_text_node!(
            commands,
            font,
            [left: pos_x, top: pos_y],
            Color::NONE,
            [[format!("{}", ev.0), 40.0, Color::YELLOW, []]],
            [CountDownTimer::new(30), PatternPopupText]
        );
    }
}

fn update_pattern_text(
    mut node_q: Query<(&mut Style, &CountDownTimer, &Children), With<PatternPopupText>>,
    mut text_q: Query<&mut Text>,
) {
    for (mut style, timer, children) in node_q.iter_mut() {
        if !timer.is_finished() {
            for &child in children.iter() {
                if let Ok(mut text) = text_q.get_mut(child) {
                    let opacity = timer.count().clamp(0, 20) as f32 / 20.0;
                    let color = &mut text.sections[0].style.color;
                    color.set_a(opacity);
                    if let Val::Px(ref mut prev_pos) = style.position.top {
                        *prev_pos -= 3.0;
                    }
                }
            }
        }
    }
}

/// ノーツ取得評価テキストを出現させる
fn spawn_catch_eval_text(
    mut commands: Commands,
    mut ev_reader: EventReader<NoteEvalEvent>,
    handles: Res<GameAssetsHandles>,
) {
    let font = handles.main_font.clone();
    for ev in ev_reader.iter() {
        // イベントに含まれているノーツ情報から評価を出現させる位置を計算.
        let get_pos_closure = |key: i32| {
            Vec2::new(
                SCREEN_WIDTH / 2.0 + KeyLane::x_coord_from_num(key) - LANE_WIDTH / 2.0,
                SCREEN_HEIGHT / 2.0 + TARGET_Y,
            )
        };
        // 出現しないならNoneを返すようにして書くのを楽にする
        let Some(Vec2 {x: pos_left, y: pos_bottom}) = (match ev.note.note_type {
            NoteType::Normal { key } => {
                Some(get_pos_closure(key))
            }
            NoteType::AdLib { key } => {
                Some(get_pos_closure(key))
            }
            NoteType::BarLine => None,
            NoteType::Long { key, length: _, id: _ } => {
                Some(get_pos_closure(key))
            }
        }) else { continue };

        if let Some(timing) = ev.eval.get_timing() {
            spawn_text_node!(
                commands,
                font,
                [left: pos_left, bottom: pos_bottom],
                Color::NONE,
                [
                    [format!("{}", timing), 15.0, timing.get_color(), []],
                    [format!("{}", ev.eval), 30.0, ev.eval.get_color(), []]
                ],
                [CountDownTimer::new(15), CatchEvalPopupText],
                {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(LANE_WIDTH), Val::Px(50.0))
                }
            );
        } else {
            spawn_text_node!(
                commands,
                font,
                [left: pos_left, bottom: pos_bottom],
                Color::NONE,
                [
                    [format!("{}", ev.eval), 30.0, ev.eval.get_color(), []]
                ],
                [CountDownTimer::new(15), CatchEvalPopupText],
                {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(LANE_WIDTH), Val::Px(50.0))
                }
            );
        }
    }
}

fn update_catch_eval_text(
    mut node_q: Query<(&mut Style, &CountDownTimer, &Children), With<CatchEvalPopupText>>,
    mut text_q: Query<&mut Text>,
) {
    for (mut style, timer, children) in node_q.iter_mut() {
        if !timer.is_finished() {
            for &child in children.iter() {
                if let Ok(mut text) = text_q.get_mut(child) {
                    let opacity = timer.count().clamp(0, 10) as f32 / 10.0;
                    let color_ref = &mut text.sections[0].style.color;
                    color_ref.set_a(opacity);
                    if let Val::Px(ref mut prev_pos) = style.position.top {
                        *prev_pos -= 3.0;
                    }
                }
            }
        }
    }
}

pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        add_enter_system!(app, Game, setup_ui);
        add_enter_system!(app, Game, setup_lane);
        // app.add_system_set(
        //     SystemSet::on_update(AppState::Game)
        //         .with_system(update_time_text.label(TimerSystemLabel::StartAudio)),
        // );
        add_update_system!(app, Game, update_score_text);
        add_update_system!(
            app,
            Game,
            update_lane_background,
            [after: TimerSystemLabel::FrameCounterUpdate]
        );
        add_update_system!(
            app,
            Game,
            spawn_pattern_text,
            [
                after: TimerSystemLabel::TimerUpdate,
                after: PatternReceptorSystemLabel::Recept
            ],
            UiSystemLabel::SpawnPatternText
        );
        add_update_system!(
            app,
            Game,
            update_pattern_text,
            [
                after: TimerSystemLabel::TimerUpdate,
                after: UiSystemLabel::SpawnPatternText
            ]
        );
        add_update_system!(
            app,
            Game,
            spawn_catch_eval_text,
            [after: TimerSystemLabel::TimerUpdate]
        );
        add_update_system!(
            app,
            Game,
            update_catch_eval_text,
            [after: TimerSystemLabel::TimerUpdate]
        );
    }
}
