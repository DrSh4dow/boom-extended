use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};

use crate::common::{
    components::{GridPosition, RelativeGridSize},
    constants::{GRID_HEIGHT, GRID_WIDTH},
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (rescale_sizes, position_translation).chain());
    }
}

fn rescale_sizes(
    mut q: Query<(&RelativeGridSize, &mut Sprite)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();

    let tile_width = window.physical_width() as f32 / GRID_WIDTH as f32;
    let tile_height = window.physical_height() as f32 / GRID_HEIGHT as f32;

    for (relative_size, mut sprite) in q.iter_mut() {
        sprite.custom_size = Some(Vec2::new(
            relative_size.width as f32 * tile_width,
            relative_size.height as f32 * tile_height,
        ))
    }
}

/// Translates the position of entities in the grid.
///
/// This function iterates over all entities that have a `GridPosition`, `Transform`, and `Sprite` component.
/// For each entity, it translates the entity's position in the grid to a position in the window.
/// The translation is done by converting the grid position (x, y) to window coordinates.
///
/// # Parameters
/// * `q`: A query that selects entities with a `GridPosition`, `Transform`, and `Sprite` component.
/// * `window`: A query that selects the primary window.
///
/// # Panics
/// This function will panic if it cannot find a primary window.
fn position_translation(
    mut q: Query<(&GridPosition, &mut Transform, &Sprite)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();

    for (pos, mut transform, sprite) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                pos.x as f32,
                window.physical_width() as f32,
                GRID_WIDTH as f32,
                sprite.anchor,
            ),
            convert(
                pos.y as f32,
                window.physical_height() as f32,
                GRID_HEIGHT as f32,
                sprite.anchor,
            ),
            transform.translation.z,
        );
    }
}

fn convert(grid_position: f32, real_size: f32, grid_size: f32, anchor: Anchor) -> f32 {
    let tile_size = real_size / grid_size;

    match anchor {
        Anchor::TopLeft => grid_position * tile_size,
        _ => (grid_position * tile_size) + tile_size / 2.,
    }
}
