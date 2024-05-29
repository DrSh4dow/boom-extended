use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Component, Debug)]
pub struct RelativeGridSize(pub f32);
