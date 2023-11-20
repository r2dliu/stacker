use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
use crate::{AppState, GameState};

use resources::*;
use systems::*;

pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), setup_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_menu);
    }
}
