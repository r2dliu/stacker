use bevy::prelude::*;

#[derive(Component)]
pub struct Board;

// Top left of board is 0, 0
#[derive(Component)]
pub struct Square {
    pub x: usize,
    pub y: usize,
}
