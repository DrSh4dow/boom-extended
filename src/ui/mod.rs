use bevy::{prelude::*, sprite::Anchor};

use crate::common::{
    components::{GridPosition, RelativeGridSize},
    constants::{GRID_HEIGHT, GRID_WIDTH},
};

pub struct GameUi;

#[derive(Component)]
struct Panel;

impl Plugin for GameUi {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui);
    }
}

#[derive(Component)]
struct ArenaGridBg;

fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // panel
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0 * 3., 32.0 * GRID_HEIGHT as f32)),
                anchor: Anchor::TopLeft,
                ..default()
            },
            texture: asset_server.load("graphics/panel.png"),
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

    // arena border
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    32.0 * GRID_WIDTH as f32 - 3.,
                    32.0 * GRID_HEIGHT as f32,
                )),
                anchor: Anchor::TopLeft,
                ..default()
            },
            texture: asset_server.load("graphics/border1.png"),
            ..default()
        },
        RelativeGridSize {
            width: GRID_WIDTH - 3,
            height: GRID_HEIGHT,
        },
        GridPosition {
            x: 3,
            y: GRID_HEIGHT,
        },
    ));

    for y in 0..GRID_HEIGHT - 2 {
        for x in 0..GRID_WIDTH - 5 {
            // arena ground
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(32.0, 32.0)),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    texture: asset_server.load("graphics/bg1.png"),
                    transform: Transform::from_xyz(0., 0., -1.),
                    ..default()
                },
                RelativeGridSize {
                    width: 1,
                    height: 1,
                },
                GridPosition { x: 4 + x, y: 2 + y },
                ArenaGridBg,
            ));
        }
    }
}
