// use bevy::prelude::*;
//
// use super::GameState;
//
// /// this plugin will display an splash screen with the logo while the game is loading
// pub fn splash_plugin(app: &mut App) {
//     app.add_systems(OnEnter(GameState::Splash), splash_setup)
//         .add_systems(Update, countdown.run_if(in_state(GameState::Splash)));
// }
//
// // Tag component used to tag entities added on the splash screen
// #[derive(Component)]
// struct OnSplashScreen;
//
// // Newtype to use a `Timer` for this screen as a resource
// #[derive(Resource, Deref, DerefMut)]
// struct SplashTimer(Timer);
//
// fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let icon = asset_server.load("graphics/boom_logo.png");
//     // Display the logo
//     commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     align_items: AlignItems::Center,
//
//                     justify_content: JustifyContent::Center,
//                     width: Val::Percent(100.0),
//                     height: Val::Percent(100.0),
//                     ..default()
//                 },
//                 ..default()
//             },
//             OnSplashScreen,
//         ))
//         .with_children(|parent| {
//             parent.spawn(ImageBundle {
//                 style: Style {
//                     // This will set the logo to be 200px wide, and auto adjust its height
//                     width: Val::Px(200.0),
//                     ..default()
//                 },
//                 image: UiImage::new(icon),
//                 ..default()
//             });
//         });
//     // Insert the timer as a resource
//     commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
// }
//
// // Tick the timer, and change state when finished
// fn countdown(
//     mut game_state: ResMut<NextState<GameState>>,
//     time: Res<Time>,
//     mut timer: ResMut<SplashTimer>,
// ) {
//     if timer.tick(time.delta()).finished() {
//         game_state.set(GameState::Menu);
//     }
// }
