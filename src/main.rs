pub mod components;
pub mod resources;

use bevy::app::AppExit;
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
        .insert_resource(resources::StepTimer(Timer::from_seconds(
            0.5_f32,
            TimerMode::Repeating,
        )))
        .add_startup_system(setup)
        .add_system(snake_head_movement_system)
        .add_system(exit_if_loose)
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
        components::Snake(vec![[0_f32, 0_f32]]),
    ));

    // set default direction
    commands.spawn(components::LastDirection(components::Direction::Left));
}

fn snake_head_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut step_timer: ResMut<resources::StepTimer>,
    mut direction_query: Query<&mut components::LastDirection>,
    mut snake_head_query: Query<(&mut components::Snake, &mut Transform)>,
) {
    // get the snake head and its transform
    let (mut snake, mut transform) = snake_head_query.single_mut();
    // get the last direction
    let mut direction = direction_query.single_mut();

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
    snake.0[0][0] = transform.translation.x;
    snake.0[0][1] = transform.translation.y;
}

fn exit_if_loose(snake_query: Query<&components::Snake>, mut exit: EventWriter<AppExit>) {
    // get the snake head
    let snake = snake_query.single();

    // check if the snake head is out of the screen
    if snake.0[0][0] < -400_f32
        || snake.0[0][0] > 800_f32
        || snake.0[0][1] < -300_f32
        || snake.0[0][1] > 600_f32
    {
        // close the game
        exit.send(AppExit)
    }
}
