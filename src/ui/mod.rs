use bevy::prelude::*;

pub struct GameUi;

impl Plugin for GameUi {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui);
    }
}

#[derive(Component)]
pub struct ArenaLayout;

#[derive(Component)]
pub struct Hud;

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::percent(15.0), GridTrack::fr(1.0)],
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                NodeBundle {
                    background_color: Color::BLUE.into(),
                    ..default()
                },
                Hud,
            ));
            commands.spawn((
                NodeBundle {
                    style: Style {
                        border: UiRect::all(Val::Px(32.)),
                        ..default()
                    },
                    border_color: Color::RED.into(),
                    ..default()
                },
                ArenaLayout,
            ));
        });
}
