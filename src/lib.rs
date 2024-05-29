mod common;
mod grid;
mod player;
mod sounds;
mod ui;

use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use grid::GridPlugin;
use player::PlayerPlugin;
use sounds::SoundsPlugin;
use ui::GameUi;

/// Runs the game application.
///
/// This function initializes a new Bevy application and adds various plugins and systems to it.
/// It sets up the default plugins with custom settings, including a custom window title, resolution, and image plugin.
/// It also adds a world inspector plugin that can be toggled with the Escape key.
/// Additionally, it adds the game-specific plugins: `GameUi`, `PlayerPlugin`, `GridPlugin`, and `SoundsPlugin`.
/// Finally, it adds a setup system that runs at startup, and then runs the application.
pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("BOOM - Extended"),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins((GameUi, PlayerPlugin, GridPlugin, SoundsPlugin))
        .add_systems(Startup, setup)
        .run();
}

/// Sets up the game window and spawns a 2D camera.
///
/// This function takes in a mutable `Commands` struct and a `Query` for the primary window.
/// It first retrieves the primary window from the query.
/// Then it spawns a 2D camera with a black clear color and a transform that centers the camera in the window.
///
/// # Parameters
/// - `commands`: A mutable `Commands` struct which is used to spawn resources and entities.
/// - `window`: A `Query` for the primary window.
fn setup(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.single();

    // spawns the camera
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: Color::BLACK.into(),
            ..default()
        },
        transform: Transform::from_xyz(
            window.physical_width() as f32 / 2.,
            window.physical_height() as f32 / 2.,
            0.,
        ),
        ..default()
    });
}
