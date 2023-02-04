use bevy::prelude::Component;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct SnakePointID(pub usize);

#[derive(Component, Debug)]
pub struct SnakePoint(pub f32, pub f32);

#[derive(Component)]
pub struct LastDirection(pub Direction);

// point number
#[derive(Component)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
