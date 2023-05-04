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
    .add_system(collision)
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
        ),
    );
}

// This controls the movement for the fighter on the Left
fn fighter_1_movement(mut fighter_position: Query<&mut Transform, With<Fighter1>>, 
    keyboard: Res<Input<KeyCode>>,
    fighter_position2: Query<&Transform, (With<Fighter2>, Without<Fighter1>)>
) {
    let fighter2_position = fighter_position2.single();


    for mut transform in &mut fighter_position {

        //The 'and' in this if statement limts Fighter 1 from to going fighter 2
        if keyboard.pressed(KeyCode::D) && transform.translation.x < fighter2_position.translation.x -18.5{
            transform.translation.x += 1.0;
        }

        if keyboard.pressed(KeyCode::A) {
            transform.translation.x -= 1.0;
        }
    }
}

//This controls the movement for the fighter on the right
fn fighter_2_movement(mut fighter_position: Query<&mut Transform, With<Fighter2>>,
    fighter2_query: Query<&Transform, (With<Fighter1>, Without<Fighter2>)>,
    keyboard: Res<Input<KeyCode>>) {

    let fighter2_position = fighter2_query.single();

    for mut transform in &mut fighter_position {
        
        //The 'and' in this if statement limts Fighter 2 from to going fighter 1
        if keyboard.pressed(KeyCode::Left) && transform.translation.x > fighter2_position.translation.x + 18.5 {
            transform.translation.x -= 1.0;
        }

        if keyboard.pressed(KeyCode::Right) {
            transform.translation.x += 1.0;
        }
    }
}

fn collision(
    fighter1_transform_query: Query<&Transform, With<Fighter1>>,
    fighter2_transform_query: Query<&Transform, With<Fighter2>>,
    mut fighter1_query: Query<&mut Fighter1, (With<Fighter1>,Without<Fighter2>)>,
    mut fighter2_query: Query<&mut Fighter2, (With<Fighter2>, Without<Fighter1>)>,
    keyboard: Res<Input<KeyCode>>
) {
    let fighter1_position = fighter1_transform_query.single();
    let fighter2_position = fighter2_transform_query.single();

    let mut f1 = fighter1_query.single_mut();
    let mut f2 = fighter2_query.single_mut();

    //Detects Collison when players are touching and Player 1 Clicks G
    if keyboard.just_released(KeyCode::G) {
        println!("Player 1 attacks");
        if fighter1_position.translation.x == fighter2_position.translation.x - 18.0{
            
            f2.health -= 10.0;
            println!("Player 1 attacked Player 2");
            println!("Player 2 health: {}", f2.health);
            if f2.health <= 0.0 {
                println!("Player 1 has won!");
                std::process::exit(1);
        }
        }
        
    }

    ////Detects Collison when players are touching and Player 2 Clicks SPACE
    if keyboard.just_released(KeyCode::Space) {
        println!("Player 2 attacks");
        if fighter2_position.translation.x == fighter1_position.translation.x + 18.0 {
            f1.health -= 10.0;
            println!("Player 2 attacks player 1");
            println!("Player 1 health: {}", f1.health);
            if f1.health <= 0.0 {
                println!("Player 2 has won!");
                std::process::exit(1);
            }
        }

    }
}





