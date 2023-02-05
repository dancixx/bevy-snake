use crate::{
    components::{Food, LastDirection, Position, SnakePoint, SnakePoints},
    utils::gen_random_position,
};
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        ..Default::default()
    });
}

pub fn setup_playground(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(816., 624.)),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn setup_snake(mut commands: Commands, mut snake: ResMut<SnakePoints>) {
    *snake = SnakePoints(vec![commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(16., 16.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Position { x: 0., y: 0. },
            SnakePoint,
        ))
        .id()]);
}

pub fn setup_first_food(mut commands: Commands) {
    let (random_x, random_y) = gen_random_position();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(16., 16.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(random_x, random_y, 0.),
                ..Default::default()
            },
            ..Default::default()
        },
        Food,
    ));
}

pub fn setup_initial_direction(mut commands: Commands) {
    commands.spawn(LastDirection::default());
}
