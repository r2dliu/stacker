use super::components::*;
use super::resources::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

pub fn setup_gui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // This plugin is responsible for rendering anything overlaid on top of the 3D game view during AppState::Playing
    // this includes things like text, the in game menus, cursor, overlays, etc.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        Gui,
    ));
}

pub fn cleanup_gui(mut commands: Commands, gui: Query<Entity, With<Gui>>) {
    for entity in gui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
