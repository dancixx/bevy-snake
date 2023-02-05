pub mod components;
pub mod resources;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::Color;
use bevy::prelude::*;
use rand::Rng;

use crate::components::Point;

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
        .add_system(controller)
        .add_system(gen_random_food)
        .add_system(move_snake)
        .add_system(render_snake)
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
            custom_size: Some(Vec2::new(816_f32, 624_f32)),
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
        components::SnakePoints::new(),
    ));

    // set default direction
    commands.spawn(components::LastDirection(components::Direction::Left));

    // add first random point
    let mut rng = rand::thread_rng();
    let random_x = f32::ceil(rng.gen_range(-400_f32..400_f32) / 16_f32) * 16_f32;
    let random_y = f32::ceil(rng.gen_range(-300_f32..300_f32) / 16_f32) * 16_f32;

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

fn move_snake(
    time: Res<Time>,
    mut step_timer: ResMut<resources::StepTimer>,
    mut direction_query: Query<&mut components::LastDirection>,
    mut snake_query: Query<&mut components::SnakePoints>,
    point_query: Query<&components::Point>,
) {
    // get the snake head and its transform
    let mut snake = snake_query.single_mut();
    let snake_head = snake.0[0];

    // get the last direction
    let direction = direction_query.single_mut();
    // get food point
    let point = point_query.single();

    let mut dx = 16_f32;
    let mut dy = 0_f32;

    match direction.0 {
        components::Direction::Up => {
            dx = 0_f32;
            dy = 16_f32;
        }
        components::Direction::Down => {
            dx = 0_f32;
            dy = -16_f32;
        }
        components::Direction::Left => {
            dx = -16_f32;
            dy = 0_f32;
        }
        components::Direction::Right => {
            dx = 16_f32;
            dy = 0_f32;
        }
    }

    let head = components::Point {
        x: snake.0[0].x + dx,
        y: snake.0[0].y + dy,
    };

    if step_timer.0.tick(time.delta()).just_finished() {
        snake.add_head(head);

        if !(snake_head.x == point.x && snake_head.y == point.y) {
            snake.remove_tail();
        } else {
            snake.push(Point {
                x: snake_head.x,
                y: snake_head.y,
            })
        }
    }
}

fn render_snake(mut commands: Commands, snake_query: Query<&components::SnakePoints>) {
    let snake = snake_query.single();

    for snake_point in snake.0.iter() {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(16_f32, 16_f32)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(snake_point.x, snake_point.y, 0_f32),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut direction: Query<&mut components::LastDirection>,
) {
    let mut direction = direction.single_mut();

    if keyboard_input.pressed(KeyCode::Up) && direction.0 != components::Direction::Down {
        direction.0 = components::Direction::Up;
    } else if keyboard_input.pressed(KeyCode::Down) && direction.0 != components::Direction::Up {
        direction.0 = components::Direction::Down;
    } else if keyboard_input.pressed(KeyCode::Left) && direction.0 != components::Direction::Right {
        direction.0 = components::Direction::Left;
    } else if keyboard_input.pressed(KeyCode::Right) && direction.0 != components::Direction::Left {
        direction.0 = components::Direction::Right;
    }
}

fn gen_random_food(
    mut point_query: Query<(&mut components::Point, &mut Transform)>,
    mut snake_query: Query<&mut components::SnakePoints>,
) {
    let (mut point, mut transform) = point_query.single_mut();
    let snake = snake_query.single_mut();
    let snake_head = snake.0[0];

    if snake_head.x == point.x && snake_head.y == point.y {
        let mut rng = rand::thread_rng();
        let random_x = f32::ceil(rng.gen_range(-400_f32..400_f32) / 16_f32) * 16_f32;
        let random_y = f32::ceil(rng.gen_range(-300_f32..300_f32) / 16_f32) * 16_f32;

        // update the point position
        transform.translation.x = random_x;
        transform.translation.y = random_y;

        // update the point details
        point.x = transform.translation.x;
        point.y = transform.translation.y;
    }
}
