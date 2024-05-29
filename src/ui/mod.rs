use bevy::prelude::*;

pub struct GameUi;

impl Plugin for GameUi {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui);
    }
}

fn spawn_game_ui(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(15.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::BLUE.into(),
            ..default()
        },
        Name::new("UI Root"),
    ));
}
