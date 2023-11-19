use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::board::BOARD_LENGTH;

// use crate::enemy::components::*;
use super::components::*;
use super::resources::*;

pub fn setup_board(mut commands: Commands) {
    println!("genearting board");
    commands.spawn((Camera3dBundle::default(), Board));
    for i in 0..BOARD_LENGTH {
        for j in 0..BOARD_LENGTH {
            commands.spawn(Square { x: i, y: j });
        }
    }
}

pub fn cleanup_board(mut commands: Commands, board: Query<Entity, With<Board>>) {
    for entity in board.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
