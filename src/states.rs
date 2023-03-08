#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum AppState {
    AssetsLoading,
    MainMenu,
    InGame,
}
