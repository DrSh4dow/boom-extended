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

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, character_movement);
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
                custom_size: Some(Vec2::new(100.0, 100.0)),
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
        Name::new("Player"),
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite, &mut PlayerMovementState)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _, mut movement_state) in &mut characters {
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
