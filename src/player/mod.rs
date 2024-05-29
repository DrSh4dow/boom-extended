mod character_animation;
mod player_components;

use bevy::prelude::*;

use crate::common::components::{GridPosition, RelativeGridSize};

use self::player_components::{
    AnimationIndices, AnimationTimer, MovementAction, PlayerMovementState, PlayerOne,
};

pub struct PlayerPlugin;

const BASE_SPEED: f32 = 140.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (character_movement, character_animation::character_animation),
        );
    }
}

// main character texture handler
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("graphics/player1.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 8, 5, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // player1
    commands.spawn((
        SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        PlayerMovementState(MovementAction::Still),
        PlayerOne,
        GridPosition { x: 0, y: 0 },
        RelativeGridSize(1.0),
        AnimationIndices {
            still: 0,
            down: (1, 7),
            up: (8, 15),
            right: (16, 23),
            left: (24, 31),
        },
        AnimationTimer(Timer::from_seconds(0.09, TimerMode::Repeating)),
        Name::new("Player"),
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &mut PlayerMovementState)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut movement_state) in &mut characters {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += BASE_SPEED * time.delta_seconds();
            movement_state.0 = MovementAction::Up;
        } else if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= BASE_SPEED * time.delta_seconds();
            movement_state.0 = MovementAction::Down;
        } else if input.pressed(KeyCode::KeyD) {
            transform.translation.x += BASE_SPEED * time.delta_seconds();
            movement_state.0 = MovementAction::Right;
        } else if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= BASE_SPEED * time.delta_seconds();
            movement_state.0 = MovementAction::Left;
        } else {
            movement_state.0 = MovementAction::Still;
        }
    }
}
