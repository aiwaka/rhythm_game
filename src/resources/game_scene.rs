use crate::AppState;

pub struct NextAppState(pub AppState);

pub struct GameCount(pub u32);

#[derive(Clone, Copy)]
pub enum GameDifficulty {
    Easy,
    Normal,
    Hard,
}
