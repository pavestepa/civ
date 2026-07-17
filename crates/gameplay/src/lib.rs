use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Boot,
    MainMenu,
    Loading,
    InGame,
    Paused,
    Settings,
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<GameplaySession>();
    }
}

#[derive(Resource, Debug, Default)]
pub struct GameplaySession {
    pub active: bool,
}
