use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
use crate::{AppState, GameState};

use resources::*;
use systems::*;

pub struct DebugOverlayPlugin;

impl Plugin for DebugOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup_debug_overlay)
            .add_systems(Update, update_debug_overlay)
            .add_systems(OnExit(AppState::Playing), cleanup_debug_overlay);
    }
}
