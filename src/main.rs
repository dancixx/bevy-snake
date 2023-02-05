mod data;
mod setups;
mod utils;

use bevy::prelude::*;
use data::SnakePoints;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Snake Game".to_string(),
                width: 1024.,
                height: 768.,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setups::setup_camera)
        .add_startup_system(setups::setup_playground)
        .add_startup_system(setups::setup_first_food)
        .add_startup_system(setups::setup_snake)
        .add_startup_system(setups::setup_initial_direction)
        .insert_resource(SnakePoints::default())
        .add_system(utils::controller)
        .run();
}

// TODO: add movement system
// TODO: add food spawning system
// TODO: add food eating system
// TODO: add snake growth system
// TODO: add game over system
