use bevy::prelude::*;

pub struct PlayerPlugin;

enum MovementAction {
    Still,
    Down,
    Left,
    Right,
    Up,
}

#[derive(Component)]
struct PlayerMovementState(MovementAction);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationIndices {
    still: usize,
    up: (usize, usize),
    down: (usize, usize),
    left: (usize, usize),
    right: (usize, usize),
}

#[derive(Component)]
struct PlayerOne;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (character_movement, character_animation));
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
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            ..default()
        },
        PlayerMovementState(MovementAction::Still),
        PlayerOne,
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
            transform.translation.y += 100.0 * time.delta_seconds();
            movement_state.0 = MovementAction::Up;
        } else if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 100.0 * time.delta_seconds();
            movement_state.0 = MovementAction::Down;
        } else if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 100.0 * time.delta_seconds();
            movement_state.0 = MovementAction::Right;
        } else if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 100.0 * time.delta_seconds();
            movement_state.0 = MovementAction::Left;
        } else {
            movement_state.0 = MovementAction::Still;
        }
    }
}

fn character_animation(
    mut player: Query<
        (
            &mut TextureAtlas,
            &PlayerMovementState,
            &AnimationIndices,
            &mut AnimationTimer,
        ),
        With<PlayerOne>,
    >,
    time: Res<Time>,
) {
    let (mut player_texture_atlas, movement_state, animation_indices, mut animation_timer) =
        player.single_mut();

    animation_timer.tick(time.delta());

    match movement_state.0 {
        MovementAction::Still => {
            player_texture_atlas.index = animation_indices.still;
        }
        MovementAction::Down => {
            if animation_timer.just_finished() {
                player_texture_atlas.index = if player_texture_atlas.index
                    >= animation_indices.down.1
                    || player_texture_atlas.index < animation_indices.down.0
                {
                    animation_indices.down.0
                } else {
                    player_texture_atlas.index + 1
                }
            }
        }
        MovementAction::Left => {
            if animation_timer.just_finished() {
                player_texture_atlas.index = if player_texture_atlas.index
                    >= animation_indices.left.1
                    || player_texture_atlas.index < animation_indices.left.0
                {
                    animation_indices.left.0
                } else {
                    player_texture_atlas.index + 1
                }
            }
        }
        MovementAction::Right => {
            if animation_timer.just_finished() {
                player_texture_atlas.index = if player_texture_atlas.index
                    >= animation_indices.right.1
                    || player_texture_atlas.index < animation_indices.right.0
                {
                    animation_indices.right.0
                } else {
                    player_texture_atlas.index + 1
                }
            }
        }
        MovementAction::Up => {
            if animation_timer.just_finished() {
                player_texture_atlas.index = if player_texture_atlas.index >= animation_indices.up.1
                    || player_texture_atlas.index < animation_indices.up.0
                {
                    animation_indices.up.0
                } else {
                    player_texture_atlas.index + 1
                }
            }
        }
    };
}
