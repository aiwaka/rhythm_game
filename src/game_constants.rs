/// ノーツの速度. ハイスピ調整に使える
pub const NOTE_BASE_SPEED: f32 = 200.0;

/// ノーツが出現するY座標. 画面外であるべき.
pub const SPAWN_POSITION: f32 = 400.0;

/// ノーツをとる判定線のY座標.
pub const TARGET_POSITION: f32 = -200.0;

/// ノーツをとるときにミスになるまでの座標誤差（TODO: 時刻誤差にすべき）
pub const THRESHOLD: f32 = 20.0;

/// 出現位置から判定線までの距離
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;
