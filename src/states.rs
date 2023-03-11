#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum AppState {
    AssetsLoading,
    MainMenu,
    DungeonCrawl,
}

//^TODO: this is not needed
#[derive(Hash, Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DungeonState {
    #[default]
    Disabled,
    Ticking,
}

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TurnState {
    Paused,
    AwaitInput,
    Running,
}
