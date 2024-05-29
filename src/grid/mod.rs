use bevy::{prelude::*, window::PrimaryWindow};

use crate::common::{
    components::{GridPosition, RelativeGridSize},
    constants::{ARENA_HEIGHT, ARENA_WIDTH},
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (rescale_sizes, position_translation).chain());
    }
}

fn convert(grid_position: f32, real_size: f32, grid_size: f32) -> f32 {
    let tile_size = real_size / grid_size;

    (grid_position * real_size / grid_size) + tile_size / 2.
}

fn rescale_sizes(
    mut q: Query<(&RelativeGridSize, &mut Sprite)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();

    let tile_width = window.physical_width() as f32 / ARENA_WIDTH as f32;
    let tile_height = window.physical_height() as f32 / ARENA_HEIGHT as f32;

    for (relative_size, mut sprite) in q.iter_mut() {
        sprite.custom_size = Some(Vec2::new(
            relative_size.0 * tile_width,
            relative_size.0 * tile_height,
        ))
    }
}

fn position_translation(
    mut q: Query<(&GridPosition, &mut Transform)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                pos.x as f32,
                window.physical_width() as f32,
                ARENA_WIDTH as f32,
            ),
            convert(
                pos.y as f32,
                window.physical_height() as f32,
                ARENA_HEIGHT as f32,
            ),
            0.0,
        );
    }
}
