/// ノーツの速度（px/秒）. ハイスピ調整に使える
pub const NOTE_BASE_SPEED: f32 = 200.0;

/// ノーツが出現するY座標. 画面外であるべき.
pub const SPAWN_POSITION: f32 = 400.0;

/// ノーツをとる判定線のY座標.
pub const TARGET_POSITION: f32 = -200.0;

/// ノーツをとるときにミスになるまでの時刻誤差（秒）
pub const ERROR_THRESHOLD: f64 = 0.1;

/// 出現位置から判定線までの距離
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;

/// ゲームステートに移行してから曲が再生されるまでの時間（秒）
pub const MUSIC_PLAY_PRECOUNT: f64 = 3.0;

/// 鍵盤レーンの幅
pub const LANE_WIDTH: f32 = 100.0;
