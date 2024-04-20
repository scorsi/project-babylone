use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}
