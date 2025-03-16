use bevy::prelude::*;

pub mod api;

pub const GAME_NAME: &str = "dc-1";

#[derive(Clone, Copy, Default, Debug, States, PartialEq, Eq, Hash)]
pub enum MainGameState {
    #[default]
    StartUp,
    StartScreen,
    SettingsScreen,
    InGame,
}

#[derive(Clone, Copy, Default, Debug, SubStates, PartialEq, Eq, Hash)]
#[source(MainGameState = MainGameState::InGame)]
pub enum InGameState {
    #[default]
    NotInGame,
    Normal,
    LevelGen,
}

#[derive(Clone, Copy, Default, Debug, SubStates, PartialEq, Eq, Hash)]
#[source(InGameState = InGameState::LevelGen)]
pub enum LevelGenState {
    #[default]
    NotGen,
    /// randommly expanding left, right, up, or down.
    Expanding,
    /// makes some rooms bigger or smaller, or funky shapes
    ModingRoomGeom,
    /// move the rooms closser, and waits for them to lock in position.
    WaitingForLock,
    /// distribute loot chests, monster spawns, and other prefabricated structures.
    SprinklePreFab,
}
