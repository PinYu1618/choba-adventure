#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum AppState {
    AssetsLoading,
    MainMenu,
    InGame,
}

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TurnState {
    Paused,
    AwaitInput,
    Running,
}
