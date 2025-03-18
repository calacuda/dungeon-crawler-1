use bevy::prelude::*;

pub mod api;
pub mod dungeon_gen;

pub const GAME_NAME: &str = "dc-1";

#[derive(Clone, Debug, Event, Default)]
pub struct GenerateLevel;

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
//
// #[derive(Clone, Copy, Default, Debug, States, PartialEq, Eq, Hash)]
// // #[source(InGameState = InGameState::LevelGen)]
// pub enum LevelGenState {
//     #[default]
//     NotGen,
//     /// randommly expanding left, right, up, or down.
//     Expanding,
//     /// makes some rooms bigger or smaller, or funky shapes
//     ModingRoomGeom,
//     /// move the rooms closser, and waits for them to lock in position.
//     WaitingForLock,
//     /// distribute loot chests, monster spawns, and other prefabricated structures.
//     SprinklePreFab,
// }

pub struct AutoTransition;

impl Plugin for AutoTransition {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, to_in_game.run_if(in_state(MainGameState::StartUp)))
            .add_systems(Update, to_expanding.run_if(in_state(MainGameState::InGame)));
    }
}

fn to_in_game(mut game_state: ResMut<NextState<MainGameState>>) {
    game_state.set(MainGameState::InGame)
}

fn to_expanding(
    mut in_game_state: ResMut<NextState<InGameState>>,
    // mut level_gen_state: ResMut<NextState<LevelGenState>>,
) {
    in_game_state.set(InGameState::LevelGen);
    // level_gen_state.set(LevelGenState::Expanding);
}
