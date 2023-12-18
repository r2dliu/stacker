use bevy::core_pipeline::fxaa::Fxaa;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_panorbit_camera::PanOrbitCamera;
use rand::prelude::*;

use crate::board::BOARD_LENGTH;

// use crate::enemy::components::*;
use super::components::*;
use super::resources::*;

pub fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("generating board");
    let camera_entity = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(
                    100. * (BOARD_LENGTH as f32) / 2.,
                    100. * (BOARD_LENGTH as f32) / 2.,
                    1000.,
                )),
                ..default()
            },
            PanOrbitCamera {
                focus: Vec3::new(
                    100. * (BOARD_LENGTH as f32) / 2.,
                    100. * (BOARD_LENGTH as f32) / 2.,
                    0.,
                ),
                ..default()
            },
            Board,
        ))
        .id();

    let mut camera = commands.entity(camera_entity);
    println!("generated camera");
    camera.insert(Fxaa::default());

    for i in 0..BOARD_LENGTH {
        for j in 0..BOARD_LENGTH {
            commands.spawn((
                Square { x: i, y: j },
                PbrBundle {
                    mesh: meshes.add(shape::Box::new(100., 100., 10.).into()),
                    material: materials.add(Color::rgb_u8(124, 144, 255).into()),
                    transform: Transform::from_xyz(100.0 * (i as f32), 100.0 * (j as f32), 0.0),
                    ..default()
                },
            ));
        }
    }
}

pub fn cleanup_board(mut commands: Commands, board: Query<Entity, With<Board>>) {
    for entity in board.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
