use bevy::core_pipeline::fxaa::Fxaa;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
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
    println!("genearting board");
    let camera_entity = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-250.0, 450.0, 900.0)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            Board,
        ))
        .id();

    let mut camera = commands.entity(camera_entity);

    camera.insert(Fxaa::default());

    for i in 0..BOARD_LENGTH {
        for j in 0..BOARD_LENGTH {
            commands.spawn((
                Square { x: i, y: j },
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 100.0 })),
                    material: materials.add(Color::rgb_u8(124, 144, 255).into()),
                    transform: Transform::from_xyz(0.0, 50.0, 0.0),
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
