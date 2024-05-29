use bevy::{audio::PlaybackMode, prelude::*};

pub struct SoundsPlugin;

#[derive(Component)]
struct MainMusic;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_music);
    }
}

fn spawn_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music/music1.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        MainMusic,
    ));
}
