/* Assets
Background - Creator: saukgp, Website: itch.io
 */


use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window { 
            resolution: (620., 360.).into(), //Setting Screen size to background size
            ..default()
        }),
        ..default()
    }))
    .add_startup_system(setup_game)
    .run()
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default()); // Spawning Camera

    commands
        .spawn(SpriteBundle{
            texture: asset_server.load("backgrounds/forest.png"), //Adding background to Screen
            ..default()
        });
}