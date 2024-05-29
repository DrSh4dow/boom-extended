mod common;
mod grid;
mod player;
mod ui;

use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use grid::GridPlugin;
use player::PlayerPlugin;
use ui::GameUi;

/// Runs the application with the default plugins
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
        .add_plugins((GameUi, PlayerPlugin, GridPlugin))
        .add_systems(Startup, setup)
        .run();
}

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
