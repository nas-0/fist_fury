use bevy::prelude::*;


use crate::components::*;

// #[derive(Component, Deref, DerefMut)]
// struct AnimationTimer(Timer);


pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>){

    commands.spawn(Camera2dBundle::default()); // Spawning Camera

    commands
        .spawn(SpriteBundle{
            texture: asset_server.load("backgrounds/forest.png"), //Adding background to Screen
            ..default()
        });

}


// pub fn spawn_fighters (mut commands: Commands, asset_server: Res<AssetServer>) {
//     //Spawning Fighter 1
//     // commands.spawn(( 
//     //     SpriteBundle{
//     //         transform: Transform::from_xyz(-200.0, -70.0, 0.0), // Assigns Translation of fighter 1 (spawn point coords)
//     //         texture: asset_server.load("sprites/characters/man_with_hat.png"),
//     //         ..default()
//     //     },
//     //     Fighter1{
//     //         health: 100.0,}
//     //     )
//     // );

//     //Spawn Fighter 2
//     // commands.spawn((

//     //     SpriteBundle{
//     //         transform: Transform::from_xyz(200.0, -70.0, 0.0), // Assigns Translation of fighter 1 (spawn point coords)
//     //         texture: asset_server.load("sprites/characters/man_with_mustache.png"),
//     //         sprite: Sprite { 
//     //             flip_x: true,
//     //                 flip_y: false,
//     //                 ..default()}, 
//     //             ..default()},
//     //     Fighter2{
//     //         health: 100.0}
//     //     ),
//     // );
// }



pub fn spawn_fighter_2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){

    let img_paths = "sprites/characters/fighter_2/fighter2_spritesheet.png".to_string();
    let texture_handle = asset_server.load(&img_paths);
    println!("SPawned");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(200.0, 202.0),
        8,
        8,
        None,
        None
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(
        (SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                flip_x: true,
                ..default()
            },
            
            transform: Transform {
                translation: Vec3 {x: 260.0, y: -80.5, z: 1.0 },
                scale: Vec3::new(2.0, 2.0, 0.0),
                ..default()
            },
            texture_atlas: texture_atlas_handle.clone(),
            ..default()},

            Fighter2{
                health: 100.0},
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Idle {has_started: true},
            MoveLeft {has_started: false},
            MoveRight {has_started: false},
            LightAttack {has_started: false},
        ),
    );
    

}

pub fn spawn_fighter_1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let img_paths = "sprites/characters/fighter_1/fighter1_spritesheet.png".to_string();

    println!("Trying to spawn player");
    let texture_handle = asset_server.load(&img_paths);
    println!("SPawned");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        // Vec2::new(210.0, 130.0),
        Vec2::new(200.0, 202.0),
        8,
        8,
        None,
        None
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(
        (SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                flip_x: false,
                ..default()
            },
            
            transform: Transform {
                translation: Vec3 {x:-250.0, y: -89.5, z: 1.0 },
                scale: Vec3::new(2.0, 2.0, 0.0),
                ..default()
            },
            texture_atlas: texture_atlas_handle.clone(),
            ..default()},
            Fighter1{
                health: 100.0},
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Idle {has_started: true},
            MoveLeft {has_started: false},
            MoveRight {has_started: false},
            LightAttack {has_started: false},
            
        )

    );

}





// This controls the movement for the fighter on the Left
pub fn fighter_1_movement(
    // mut fighter_position: Query<&mut Transform, With<Fighter1>>, 
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Transform,
        &mut LightAttack,
        &mut MoveRight,
        &mut MoveLeft,
        &mut Idle,
    ), With<Fighter1>>,
    fighter_position2: Query<&Transform, (With<Fighter2>, Without<Fighter1>)>
) {
    let fighter2_position = fighter_position2.single();

    // for mut transform in &mut fighter_position {
    //     println!("Fighter 1: {}", transform.translation.x);
    //     //The 'and' in this if statement limits Fighter 1 from going past fighter 2
    //     if keyboard.pressed(KeyCode::D) && transform.translation.x < fighter2_position.translation.x -18.5{
    //         transform.translation.x += 1.0;
    //     }

    //     if keyboard.pressed(KeyCode::A) {
    //         transform.translation.x -= 1.0;
    //     }
    // }
    for (mut timer, mut sprite, mut sprite_transform, mut light_attack, mut move_right, mut move_left,mut idle_movement) in &mut query {
        // println!("Current index is {}", sprite.index);
        println!("Fighter 1: {}", sprite_transform.translation.x);
        timer.tick(time.delta());
        
        if !light_attack.has_started && keyboard.just_pressed (KeyCode::G) {
          light_attack.has_started = true;
          idle_movement.has_started = false
        }

        if !move_right.has_started && keyboard.just_pressed (KeyCode::D) {
            move_right.has_started = true;
            idle_movement.has_started = false;
          }

          if !move_right.has_started && keyboard.just_pressed (KeyCode::A) {
            move_left.has_started = true;
            idle_movement.has_started = false;
          }
        // If the attack animation component is set to true the animation goes
        if light_attack.has_started {
          if timer.just_finished() {
              //count = count + 1;
              if sprite.index > 5 || sprite.index < 0 {
                    sprite.index = 0;
              }else if sprite.index == 5 {
                    sprite.index = 32;
                    light_attack.has_started = false;
                    idle_movement.has_started = true;
                } else {
                    sprite.index +=1;
                }
          }
        } else if move_right.has_started {
            if keyboard.pressed(KeyCode::D){
                
                if timer.just_finished() {
                    //count = count + 1;
                    if sprite.index > 47 || sprite.index < 40 {
                        sprite.index = 40;
                    }else if sprite.index == 47 {
                        sprite.index = 40;
                        // move_right.has_started = false;
                        // idle_movement.has_started = true;
                    } else {
                        sprite.index +=1;
                        if sprite_transform.translation.x < fighter2_position.translation.x - 18.5{
                            sprite_transform.translation.x += 10.0;
                        }

                    }
                }
            }else{
                move_right.has_started = false;
                idle_movement.has_started = true;
            }
        
        } else if move_left.has_started {
            
            if keyboard.pressed(KeyCode::A) {
                
                if timer.just_finished() {
                    //count = count + 1;
                    if sprite.index > 47 || sprite.index < 40 {
                        sprite.index = 47;
                    }else if sprite.index == 40 {
                        sprite.index = 47;
                        // move_right.has_started = false;
                        // idle_movement.has_started = true;
                    } else {
                        sprite.index -=1;
                        if sprite_transform.translation.x >= -270.0{
                            sprite_transform.translation.x -= 10.0;
                        }
                        
                        
                    }
                }

            } else {
                move_left.has_started = false;
                idle_movement.has_started = true;
            }


        }else if idle_movement.has_started {
            if timer.just_finished() {
                if sprite.index > 31 || sprite.index < 24{
                    sprite.index = 24;
                }else if sprite.index == 31 {
                    sprite.index = 24; 
                } else {
                    sprite.index +=1;
                }
            }
        }
    }
}

//This controls the movement for the fighter on the right
pub fn fighter_2_movement(
    fighter_position1: Query<&Transform, (With<Fighter1>, Without<Fighter2>)>,
    // mut fighter_position: Query<&mut Transform, With<Fighter2>>,
    // mut fighter_sprite_query: Query<&TextureAtlasSprite, With<Fighter2>>,
    // assets: Res<AssetServer>,
    // fighter2_query: Query<&Transform, (With<Fighter1>, Without<Fighter2>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Transform,
        &mut LightAttack,
        &mut MoveRight,
        &mut MoveLeft,
        &mut Idle,
    ), With<Fighter2>>,
) {
    let fighter1_position = fighter_position1.single();
    // let fighter2_position = fighter2_query.single();
    // let mut fighter_sprite = fighter_sprite_query.single_mut();



    // for mut transform in &mut fighter_position {
        
    //     //The 'and' in this if statement limits Fighter 2 from going past fighter 1
    //     if keyboard.pressed(KeyCode::Left) && transform.translation.x > fighter2_position.translation.x + 18.5 {
    //         transform.translation.x -= 1.0;
    //     }

    //     if keyboard.pressed(KeyCode::Right) {
    //         transform.translation.x += 1.0;
    //     }
    // }

    for (mut timer, mut sprite, mut sprite_transform, mut light_attack, mut move_right, mut move_left,mut idle_movement) in &mut query {
        // println!("Current index is {}", sprite.index);
        //println!("Fighter 2: {}", sprite.index);
        timer.tick(time.delta());
        
        if !light_attack.has_started && keyboard.just_pressed (KeyCode::L) {
          light_attack.has_started = true;
          idle_movement.has_started = false
        }

        if !move_right.has_started && keyboard.just_pressed (KeyCode::Right) {
            move_right.has_started = true;
            idle_movement.has_started = false;
          }

          if !move_right.has_started && keyboard.just_pressed (KeyCode::Left) {
            move_left.has_started = true;
            idle_movement.has_started = false;
          }
        // If the attack animation component is set to true the animation goes
        if light_attack.has_started {
          if timer.just_finished() {
              //count = count + 1;
              if sprite.index > 3 || sprite.index < 0 {
                    sprite.index = 0;
              }else if sprite.index == 3 {
                    sprite.index = 32;
                    light_attack.has_started = false;
                    idle_movement.has_started = true;
                } else {
                    sprite.index +=1;
                }
          }
        } else if move_right.has_started {
            if keyboard.pressed(KeyCode::Right){
                
                if timer.just_finished() {
                    //count = count + 1;
                    if sprite.index > 55 || sprite.index < 48 {
                        sprite.index = 55;
                    }else if sprite.index == 48 {
                        sprite.index = 55;
                        // move_right.has_started = false;
                        // idle_movement.has_started = true;
                    } else {
                        sprite.index -=1;
                        if sprite_transform.translation.x <= 270.0{
                            sprite_transform.translation.x += 10.0;
                        }

                    }
                }
            }else{
                move_right.has_started = false;
                idle_movement.has_started = true;
            }
        
        } else if move_left.has_started {
            
            if keyboard.pressed(KeyCode::Left) {
                
                if timer.just_finished() {
                    //count = count + 1;
                    if sprite.index > 55 || sprite.index < 48 {
                        sprite.index = 48;
                    }else if sprite.index == 55 {
                        sprite.index = 48;
                        // move_right.has_started = false;
                        // idle_movement.has_started = true;
                    } else {
                        sprite.index +=1;
                        if sprite_transform.translation.x > fighter1_position.translation.x +18.5 {
                            sprite_transform.translation.x -= 10.0;
                        }
                        
                    }
                }

            } else {
                move_left.has_started = false;
                idle_movement.has_started = true;
            }


        }else if idle_movement.has_started {
            if timer.just_finished() {
                if sprite.index > 35 || sprite.index < 32{
                    sprite.index = 32;
                }else if sprite.index == 35 {
                    sprite.index = 32; 
                } else {
                    sprite.index +=1;
                }
            }
        }
    }



}


pub fn collision(
    fighter1_transform_query: Query<&Transform, With<Fighter1>>,
    fighter2_transform_query: Query<&Transform, With<Fighter2>>,
    mut fighter1_query: Query<&mut Fighter1, (With<Fighter1>,Without<Fighter2>)>,
    mut fighter2_query: Query<&mut Fighter2, (With<Fighter2>, Without<Fighter1>)>,
    keyboard: Res<Input<KeyCode>>,
) {
    let fighter1_position = fighter1_transform_query.single();
    let fighter2_position = fighter2_transform_query.single();

    let mut f1 = fighter1_query.single_mut();
    let mut f2 = fighter2_query.single_mut();

    // let mut fighter_sprite = fighter_sprite_query.single_mut();

    //Detects Collison when players are touching and Player 1 Clicks G
    if keyboard.just_released(KeyCode::G) {
        println!("Player 1 attacks");
        
        // If the player presses the Attack button while touching the other player they inflict damage
        if fighter1_position.translation.x >= fighter2_position.translation.x - 170.0{
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
    if keyboard.just_released(KeyCode::L) {
        println!("Player 2 attacks");

 
        // fighter_sprite.index = 2;

        // fighter_sprite.index = 0;



        // If the player presses the Attack button while touching the other player they inflict damage
        if fighter2_position.translation.x <= fighter1_position.translation.x + 170.0 {
            f1.health -= 10.0;
            println!("Player 2 attacks player 1");
            println!("Player 1 health: {}", f1.health);
            // fighter_sprite.index = 0;
            if f1.health <= 0.0 {
                println!("Player 2 has won!");
                std::process::exit(1);
            }
        }

    }
}

pub fn spawn_health_bar (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let img_path = "sprites/healthbar_spritesheet.png".to_string();
    let img_path = "sprites/healthbars/healthbar_sprite_sheet.png".to_string();
    
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

pub fn change_health_bar (
    mut fighter1_healthbar_query: Query<&mut TextureAtlasSprite, (With<Fighter1Health>, Without<Fighter2Health>)>,
    mut fighter1_query: Query<&mut Fighter1, (With<Fighter1>,Without<Fighter2>)>,
    mut fighter2_healthbar_query: Query<&mut TextureAtlasSprite, (With<Fighter2Health>, Without<Fighter1Health>)>,
    mut fighter2_query: Query<&mut Fighter2, (With<Fighter2>,Without<Fighter1>)>,
){

    // Fighter 1 Healthbar
    let mut fighter1_healthbar = fighter1_healthbar_query.get_single_mut().unwrap();
    let f1 = fighter1_query.single_mut();
    
    if f1.health == 90.0 {
        fighter1_healthbar.index = 4;
    }

    if f1.health == 70.0 {
        fighter1_healthbar.index = 3;
    }

    if f1.health == 50.0 {
        fighter1_healthbar.index = 2;
    }

    if f1.health == 20.0 {
        fighter1_healthbar.index = 1;
    }

    if f1.health == 0.0 {
        fighter1_healthbar.index = 0;
    }

    //Fighter 2 HealthBar
    let mut fighter2_healthbar = fighter2_healthbar_query.get_single_mut().unwrap();

    let f2 = fighter2_query.single_mut();
    
    if f2.health == 90.0 {
        fighter2_healthbar.index = 4;
    }

    if f2.health == 70.0 {
        fighter2_healthbar.index = 3;
    }

    if f2.health == 50.0 {
        fighter2_healthbar.index = 2;
    }

    if f2.health == 20.0 {
        fighter2_healthbar.index = 1;
    }

    if f2.health == 0.0 {
        fighter2_healthbar.index = 0;
    }

}

