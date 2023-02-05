mod components;
mod resources;
mod setups;
mod utils;

use bevy::prelude::Color;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use components::SnakePoints;

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
        .add_startup_system(setups::setup_snake)
        .add_startup_system(setups::setup_first_food)
        .add_startup_system(setups::setup_initial_direction)
        .insert_resource(SnakePoints::default())
        .insert_resource(resources::StepTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .add_system(utils::controller)
        // .add_system(gen_random_food)
        // .add_system(move_snake)
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(0.5))
        //         .with_system(render_snake),
        // )
        .run();
}

// fn move_snake(
//     time: Res<Time>,
//     mut step_timer: ResMut<resources::StepTimer>,
//     mut direction_query: Query<&mut components::LastDirection>,
//     mut snake_query: Query<&mut components::SnakePoints>,
//     point_query: Query<&components::Point>,
// ) {
//     // get the snake head and its transform
//     let mut snake = snake_query.single_mut();
//     let snake_head = snake.head();

//     // get the last direction
//     let direction = direction_query.single_mut();
//     // get food point
//     let point = point_query.single();

//     let mut dx = 16.;
//     let mut dy = 0.;

//     match direction.0 {
//         components::Direction::Up => {
//             dx = 0.;
//             dy = 16.;
//         }
//         components::Direction::Down => {
//             dx = 0.;
//             dy = -16.;
//         }
//         components::Direction::Left => {
//             dx = -16.;
//             dy = 0.;
//         }
//         components::Direction::Right => {
//             dx = 16.;
//             dy = 0.;
//         }
//     }

//     let head = [snake.0[0][0] + dx, snake.0[0][1] + dy];

//     if step_timer.0.tick(time.delta()).just_finished() {
//         snake.add_head(head);

//         if snake_head[0] == point.x && snake_head[1] == point.y {
//             snake.push([snake_head[0], snake_head[1]]);
//         } else {
//             snake.remove_tail();
//         }
//     }
// }

// fn gen_random_food(
//     mut point_query: Query<(&mut components::Point, &mut Transform)>,
//     mut snake_query: Query<&mut components::SnakePoints>,
// ) {
//     let (mut point, mut transform) = point_query.single_mut();
//     let snake = snake_query.single_mut();
//     let snake_head = snake.head();

//     if snake_head[0] == point.x && snake_head[1] == point.y {
//         let (random_x, random_y) = gen_random_position();

//         // update the point position
//         transform.translation.x = random_x;
//         transform.translation.y = random_y;

//         // update the point details
//         point.x = transform.translation.x;
//         point.y = transform.translation.y;
//     }
// }
