// mod splash;
mod player;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use player::PlayerPlugin;

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
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        // .insert_resource(Volume(7))
        // .init_state::<GameState>()
        // adds the plugins for each state
        // .add_plugins(splash::splash_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    // spawns the camera
    commands.spawn(Camera2dBundle::default());
}
