pub mod components;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::Color;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Snake Game".to_string(),
                width: 800_f32,
                height: 600_f32,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setup)
        .add_system(snake_head_movement_system)
        .run();
}

fn setup(mut commands: Commands) {
    // set background color
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        ..Default::default()
    });

    // render the first point as a circle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(16_f32, 16_f32)),
                ..Default::default()
            },
            ..Default::default()
        },
        components::SnakeHead { x: 0_f32, y: 0_f32 },
    ));

    // add the snake head
}

fn snake_head_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut components::SnakeHead, &mut Transform)>,
) {
    let (mut head, mut transform) = query.single_mut();

    if keyboard_input.pressed(KeyCode::Up) {
        transform.translation.y += 10_f32;
    } else if keyboard_input.pressed(KeyCode::Down) {
        transform.translation.y -= 10_f32;
    } else if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x -= 10_f32;
    } else if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += 10_f32;
    }

    head.x = transform.translation.x;
    head.y = transform.translation.y;
}
