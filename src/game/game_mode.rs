#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Menu,
    Playing,
    GameOver,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Menu
    }
}
