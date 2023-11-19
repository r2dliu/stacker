use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
use crate::AppState;

use resources::*;
use systems::*;

pub const BOARD_LENGTH: usize = 6;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup_board)
            .add_systems(OnExit(AppState::Playing), cleanup_board);
    }
}
