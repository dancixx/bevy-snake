use crate::components::{self, Position, SnakePoint};
use bevy::prelude::*;
use rand::Rng;

pub fn gen_random_position() -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let random_x = f32::ceil(rng.gen_range(-(400.)..400.) / 16.) * 16.;
    let random_y = f32::ceil(rng.gen_range(-(300.)..(300.)) / 16.) * 16.;

    (random_x, random_y)
}

pub fn spawn_snake_point(mut command: Commands, position: Position) -> Entity {
    command
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(16., 16.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            position,
            SnakePoint,
        ))
        .id()
}

pub fn controller(
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
