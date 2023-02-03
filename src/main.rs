pub mod components;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::Color;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Snake Game".to_string(),
                width: 800f32,
                height: 600f32,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add the window size resource
    commands.insert_resource(components::WindowSize {
        width: 800f32,
        height: 600f32,
    });

    // set background color
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GREEN),
        },
        ..Default::default()
    });

    let random_x = rand::random::<f32>() * 800.0;
    let random_y = rand::random::<f32>() * 600.0;

    // add the first point
    commands.spawn((
        components::Point(0),
        components::PositionX(random_x),
        components::PositionY(random_y),
    ));

    // render the first point as a circle
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black_dot.png"),
        transform: Transform {
            translation: Vec3::new(
                f32::abs(800f32 / 2f32 - random_x),
                f32::abs(600f32 / 2f32 - random_y),
                0.0,
            ),
            scale: Vec3::new(0.1, 0.1, 0.1),
            ..Default::default()
        },
        ..Default::default()
    });
}
