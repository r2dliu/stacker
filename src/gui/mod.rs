use bevy::prelude::*;

use crate::AppState;

pub mod components;
pub mod debug_overlay;
pub mod in_game_menu;
pub mod menu;
pub mod resources;
mod systems;
use systems::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup_gui)
            .add_systems(OnExit(AppState::Playing), cleanup_gui);
    }
}
