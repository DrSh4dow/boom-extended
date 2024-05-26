// mod splash;

use bevy::prelude::*;

// #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
// enum GameState {
//     #[default]
//     Splash,
//     Menu,
//     Game,
// }

/// It will be a resource in the app
// #[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
// struct Volume(u32);

enum MovementAnimation {
    Still,
    Down,
    Left,
    Right,
    Up,
}

#[derive(Component)]
struct PlayerMovementState(MovementAnimation);

/// Runs the application with the default plugins
pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Logic Farming Rougelike"),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        // .insert_resource(Volume(7))
        // .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        // adds the plugins for each state
        // .add_plugins(splash::splash_plugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // spawns the camera
    commands.spawn(Camera2dBundle::default());

    // main character texture handler
    let texture = asset_server.load("graphics/player1.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 8, 5, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // player1
    commands
        .spawn(SpriteSheetBundle {
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
        })
        .insert(PlayerMovementState(MovementAnimation::Still));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite, &mut PlayerMovementState)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _, mut movement_state) in &mut characters {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 100.0 * time.delta_seconds();
            movement_state.0 = MovementAnimation::Up;
        } else if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 100.0 * time.delta_seconds();
            movement_state.0 = MovementAnimation::Down;
        } else if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 100.0 * time.delta_seconds();
            movement_state.0 = MovementAnimation::Right;
        } else if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 100.0 * time.delta_seconds();
            movement_state.0 = MovementAnimation::Left;
        } else {
            movement_state.0 = MovementAnimation::Still;
        }
    }
}
