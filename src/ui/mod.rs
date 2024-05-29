use bevy::{prelude::*, sprite::Anchor};

use crate::common::{
    components::{GridPosition, RelativeGridSize},
    constants::GRID_HEIGHT,
};

pub struct GameUi;

#[derive(Component)]
struct Panel;

impl Plugin for GameUi {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui);
    }
}

fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let panel_handler = asset_server.load("graphics/panel.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0 * 3., 32.0 * GRID_HEIGHT as f32)),
                anchor: Anchor::TopLeft,
                ..default()
            },
            texture: panel_handler,
            ..default()
        },
        Panel,
        RelativeGridSize {
            width: 3,
            height: GRID_HEIGHT,
        },
        GridPosition {
            x: 0,
            y: GRID_HEIGHT,
        },
    ));
}
