use bevy::prelude::Component;

#[derive(Component)]
pub struct Snake(pub Vec<[f32; 2]>);

#[derive(Component)]
pub struct LastDirection(pub Direction);

// point number
#[derive(Component)]
pub struct Point(pub usize);

// x position of the point
#[derive(Component)]
pub struct PositionX(pub f32);

// y position of the point
#[derive(Component)]
pub struct PositionY(pub f32);

#[derive(Component, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
