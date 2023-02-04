pub mod components;
pub mod resources;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::Color;
use bevy::prelude::*;
use components::SnakePointID;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Snake Game".to_string(),
                width: 1024_f32,
                height: 768_f32,
                ..Default::default()
            },
            ..Default::default()
        }))
        .insert_resource(resources::StepTimer(Timer::from_seconds(
            0.5_f32,
            TimerMode::Repeating,
        )))
        .add_startup_system(setup)
        .add_system(snake_head_movement_system)
        .add_system(build_snake_body)
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

    // draw the border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(800_f32, 600_f32)),
            ..Default::default()
        },
        ..Default::default()
    });

    // add the snake head
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(16_f32, 16_f32)),
                ..Default::default()
            },
            ..Default::default()
        },
        components::SnakePointID(0_usize),
        components::SnakePoint(0_f32, 0_f32),
    ));

    // set default direction
    commands.spawn(components::LastDirection(components::Direction::Left));

    // add first random point
    let random_x = f32::ceil((rand::random::<f32>() * 800_f32 - 416_f32) / 16_f32) * 16_f32;
    let random_y = f32::ceil((rand::random::<f32>() * 800_f32 - 316_f32) / 16_f32) * 16_f32;

    // add the point
    commands.spawn((
        SpriteBundle {
            // set the color of the point
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(16_f32, 16_f32)),
                ..Default::default()
            },
            // set the position of the point
            transform: Transform {
                translation: Vec3::new(random_x, random_y, 0_f32),
                ..Default::default()
            },
            ..Default::default()
        },
        // store point details
        components::Point {
            x: random_x,
            y: random_y,
        },
    ));
}

fn snake_head_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut step_timer: ResMut<resources::StepTimer>,
    mut direction_query: Query<&mut components::LastDirection>,
    mut snake_head_query: Query<(
        &mut components::SnakePoint,
        &components::SnakePointID,
        &mut Transform,
    )>,
) {
    // get the snake head and its transform
    let (mut snake, id, mut transform) = snake_head_query.single_mut();
    // get the last direction
    let mut direction = direction_query.single_mut();

    // check if the snake head is out of the screen and move it to the other side
    if snake.0 >= 416_f32 {
        transform.translation.x = -400_f32;
        snake.0 = -400_f32;
    }

    if snake.0 <= -416_f32 {
        transform.translation.x = 400_f32;
        snake.0 = 400_f32;
    }

    if snake.1 >= 304_f32 {
        transform.translation.y = -300_f32;
        snake.1 = -300_f32;
    }

    if snake.1 <= -304_f32 {
        transform.translation.y = 300_f32;
        snake.1 = 300_f32;
    }

    // check if the user pressed a key and change the direction
    if keyboard_input.pressed(KeyCode::Up) {
        direction.0 = components::Direction::Up;
    } else if keyboard_input.pressed(KeyCode::Down) {
        direction.0 = components::Direction::Down;
    } else if keyboard_input.pressed(KeyCode::Left) {
        direction.0 = components::Direction::Left;
    } else if keyboard_input.pressed(KeyCode::Right) {
        direction.0 = components::Direction::Right;
    }

    // move the snake head
    if step_timer.0.tick(time.delta()).just_finished() {
        match direction.0 {
            components::Direction::Up => transform.translation.y += 16_f32,
            components::Direction::Down => transform.translation.y -= 16_f32,
            components::Direction::Left => transform.translation.x -= 16_f32,
            components::Direction::Right => transform.translation.x += 16_f32,
        }
    }

    // update the snake head position
    snake.0 = transform.translation.x;
    snake.1 = transform.translation.y;
}

fn build_snake_body(
    mut commands: Commands,
    mut point_query: Query<(&mut components::Point, &mut Transform)>,
    snake_query: Query<(&mut components::SnakePoint, &SnakePointID)>,
) {
    let (mut point, mut transform) = point_query.single_mut();

    for (snake_point, id) in snake_query.iter() {
        if id == &components::SnakePointID(0_usize)
            && snake_point.0 == point.x
            && snake_point.1 == point.y
        {
            // get the last snake point
            let last_snake_point = snake_query.iter().last().expect("No snake points found");
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(16_f32, 16_f32)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            (last_snake_point.0).0 + 16_f32,
                            (last_snake_point.0).1 + 16_f32,
                            0_f32,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                components::SnakePointID(&id.0 + 1),
                components::SnakePoint(
                    (last_snake_point.0).0 + 16_f32,
                    (last_snake_point.0).1 + 16_f32,
                ),
            ));

            let random_x = f32::ceil((rand::random::<f32>() * 800_f32 - 400_f32) / 16_f32) * 16_f32;
            let random_y = f32::ceil((rand::random::<f32>() * 800_f32 - 400_f32) / 16_f32) * 16_f32;

            // update the point position
            transform.translation.x = random_x;
            transform.translation.y = random_y;

            // update the point details
            point.x = transform.translation.x;
            point.y = transform.translation.y;
        }
    }
}
