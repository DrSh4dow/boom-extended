use bevy::prelude::*;

pub enum MovementAction {
    Still,
    Down,
    Left,
    Right,
    Up,
}

#[derive(Component)]
pub struct PlayerMovementState(pub MovementAction);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub still: usize,
    pub up: (usize, usize),
    pub down: (usize, usize),
    pub left: (usize, usize),
    pub right: (usize, usize),
}

#[derive(Component)]
pub struct PlayerOne;
