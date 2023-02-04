use bevy::prelude::Component;

// snake head
#[derive(Component)]
pub struct SnakeHead {
    pub x: f32,
    pub y: f32,
}

// point number
#[derive(Component)]
pub struct Point(pub usize);

// x position of the point
#[derive(Component)]
pub struct PositionX(pub f32);

// y position of the point
#[derive(Component)]
pub struct PositionY(pub f32);

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
