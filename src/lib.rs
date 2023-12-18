#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod board;
mod gui;
mod loading;
mod player;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::board::BoardPlugin;
use crate::gui::in_game_menu::InGameMenuPlugin;
use crate::gui::menu::MenuPlugin;
use crate::gui::GuiPlugin;
use crate::loading::LoadingPlugin;
use crate::player::PlayerPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::{app::App, core_pipeline::experimental::taa::TemporalAntiAliasPlugin};
use gui::debug_overlay::DebugOverlayPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    Playing,
    Menu,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    Completed,
}

pub struct StackerPlugin;

impl Plugin for StackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_state::<GameState>()
            .add_plugins((
                LoadingPlugin,
                DebugOverlayPlugin,
                MenuPlugin,
                BoardPlugin,
                GuiPlugin,
                InGameMenuPlugin,
                ActionsPlugin,
                InternalAudioPlugin,
                PlayerPlugin,
            ));

        #[cfg(debug_assertions)]
        {
            // app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
