/* Assets
Background - Creator: saukgp, Website: itch.io
Sounds - https://mixkit.co/free-sound-effects/fight/
 */
use bevy::prelude::*;

// #[derive(Clone)]
// Handle<T> implements Clone;

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
    .add_startup_system(spawn_health_bar)
    .add_system(fighter_1_movement)
    .add_system(fighter_2_movement)
    .add_system(collision)
    .add_system(change_health_bar)
    .run()
}




fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){

    commands.spawn(Camera2dBundle::default()); // Spawning Camera
    
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 6, None, None);

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        }
    );


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
        //The 'and' in this if statement limits Fighter 1 from going past fighter 2
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
        
        //The 'and' in this if statement limits Fighter 2 from going past fighter 1
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
        
        // If the player presses the Attack button while touching the other player they inflict damage
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

        // If the player presses the Attack button while touching the other player they inflict damage
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

#[derive(Component)]
struct Fighter1Health {
}

#[derive(Component)]
struct Fighter2Health {
}

fn spawn_health_bar (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let img_path = "sprites/healthbar_spritesheet.png".to_string();
    
    let texture_handle = asset_server.load(&img_path);

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(178.5, 50.5), 
        1,
        6,
        None,
        None
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(
            (SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(5),
                transform: Transform { 
                    translation: Vec3{ x: -235.0, y: 150.0, z: 0.0 }, 
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                ..default()},
            Fighter1Health{
            }, )
            );
        println!("spawning");
    commands
        .spawn(
            (SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(5),
                transform: Transform { 
                    translation: Vec3{ x: 235.0, y: 150.0, z: 0.0 }, 
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                ..default()},
            Fighter2Health{
            }, )
            );

}

fn change_health_bar (
    mut fighter1_healthbar_query: Query<&mut TextureAtlasSprite, (With<Fighter1Health>, Without<Fighter2Health>)>,
    mut fighter1_query: Query<&mut Fighter1, (With<Fighter1>,Without<Fighter2>)>,
    mut fighter2_healthbar_query: Query<&mut TextureAtlasSprite, (With<Fighter2Health>, Without<Fighter1Health>)>,
    mut fighter2_query: Query<&mut Fighter2, (With<Fighter2>,Without<Fighter1>)>,
){

    // Fighter 1 Healthbar
    let mut fighter1_healthbar = fighter1_healthbar_query.get_single_mut().unwrap();
    let f1 = fighter1_query.single_mut();
    
    if f1.health == 80.0 {
        fighter1_healthbar.index = 4;
    }

    if f1.health == 60.0 {
        fighter1_healthbar.index = 3;
    }

    if f1.health == 30.0 {
        fighter1_healthbar.index = 2;
    }

    if f1.health == 10.0 {
        fighter1_healthbar.index = 1;
    }

    if f1.health == 0.0 {
        fighter1_healthbar.index = 0;
    }

    //Fighter 2 HealthBar
    let mut fighter2_healthbar = fighter2_healthbar_query.get_single_mut().unwrap();

    let f2 = fighter2_query.single_mut();
    
    if f2.health == 80.0 {
        fighter2_healthbar.index = 4;
    }

    if f2.health == 60.0 {
        fighter2_healthbar.index = 3;
    }

    if f2.health == 30.0 {
        fighter2_healthbar.index = 2;
    }

    if f2.health == 10.0 {
        fighter2_healthbar.index = 1;
    }

    if f2.health == 0.0 {
        fighter2_healthbar.index = 0;
    }

}