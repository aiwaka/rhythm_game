/// ゲームのフレームレート
pub const FRAMERATE: f64 = 60.0;

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

pub const BASIC_NOTE_SPEED: f32 = 200.0;

/// ノーツが出現するY座標. 画面外であるべき.
pub const NOTE_SPAWN_Y: f32 = 400.0;

/// ノーツをとる判定線のY座標.
pub const TARGET_Y: f32 = -200.0;

/// ノーツをとるときにミスになるまでの時刻誤差（秒）
pub const MISS_THR: f64 = 0.1;

/// 出現位置から判定線までの距離
pub const DISTANCE: f32 = TARGET_Y - NOTE_SPAWN_Y;

/// ゲームステートに移行してから曲が再生されるまでの時間（秒）
pub const MUSIC_PLAY_PRECOUNT: f64 = 4.0;

/// 鍵盤レーンの幅（px）
pub const LANE_WIDTH: f32 = 100.0;
