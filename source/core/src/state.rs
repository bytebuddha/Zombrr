#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum ZombrrState {
    Booting,
    Menu(MenuState),
    Arena(ArenaState),
}

impl Default for ZombrrState {
    fn default() -> ZombrrState {
        ZombrrState::Booting
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum MenuState {
    Loading,
    Select,
    Configure,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum ArenaState {
    Loading,
    Playing,
    Over,
}
