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
    .add_startup_system(spawn_fighters)
    .add_system(fighter_1_movement)
    .add_system(fighter_2_movement)
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

#[derive(Component)]
struct Fighter1 { // Fighter 1 component
    health: f32,
}

#[derive(Component)]
struct Fighter2 { // Fighter 2 component
    health: f32,
}


fn spawn_fighters (mut commands: Commands, asset_server: Res<AssetServer>) {
    //Spawning Fighter 1
    commands.spawn(( 
        SpriteBundle{
            transform: Transform::from_xyz(-200.0, -70.0, 0.0), // Assigns Translation of fighter 1 (spawn point coords)
            texture: asset_server.load("sprites/man_with_hat.png"),
            ..default()
        },
        Fighter1{
            health: 100.0,}
    )
    );

    //Spawn Fighter 2
    commands.spawn((
         
        SpriteBundle{
            transform: Transform::from_xyz(200.0, -70.0, 0.0), // Assigns Translation of fighter 1 (spawn point coords)
            texture: asset_server.load("sprites/man_with_mustache.png"),
            sprite: Sprite { 
                flip_x: true,
                    flip_y: false,
                    ..default()}, 
                ..default()},
        Fighter2{
            health: 100.0}
        )
    );
}

// This controls the movement for the fighter on the Left
fn fighter_1_movement(mut fighter_position: Query<&mut Transform, With<Fighter1>>, keyboard: Res<Input<KeyCode>>) {
    for mut transform in &mut fighter_position {
        if keyboard.pressed(KeyCode::D) {
            transform.translation.x += 1.0;
        }

        if keyboard.pressed(KeyCode::A) {
            transform.translation.x -= 1.0;
        }
    }
}

//This controls the movement for the fighter on the right
fn fighter_2_movement(mut fighter_position: Query<&mut Transform, With<Fighter2>>, keyboard: Res<Input<KeyCode>>) {
    for mut transform in &mut fighter_position {
        if keyboard.pressed(KeyCode::Left) {
            transform.translation.x -= 1.0;
        }

        if keyboard.pressed(KeyCode::Right) {
            transform.translation.x += 1.0;
        }
    }
}

