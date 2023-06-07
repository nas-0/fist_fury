/* Assets
Background - Creator: saukgp, Website: itch.io
Sounds - https://mixkit.co/free-sound-effects/fight/
 */

pub mod components;
mod systems;

use systems::*;

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
    .add_startup_system(spawn_fighter_2)
    .add_startup_system(spawn_health_bar)
    .add_startup_system(spawn_fighter_1)
    .add_system(fighter_1_movement)
    .add_system(fighter_2_movement)
    .add_system(collision)
    .add_system(change_health_bar)
    .run()
}




