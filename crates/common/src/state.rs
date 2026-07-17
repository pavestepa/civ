use serde::{Deserialize, Serialize};

/// Top-level application states shared between engine and launcher.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppState {
    #[default]
    Boot,
    MainMenu,
    Loading,
    InGame,
    Paused,
    Settings,
}
