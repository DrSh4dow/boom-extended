mod splash;

use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

/// It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

/// Runs the application with the default plugins
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Volume(7))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        // adds the plugins for each state
        .add_plugins(splash::splash_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
