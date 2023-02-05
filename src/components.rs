use bevy::prelude::{Component, Deref, DerefMut, Entity, Resource};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakePoints(pub Vec<Entity>);

#[derive(Component)]
pub struct SnakePoint;

#[derive(Component)]
pub struct LastDirection(pub Direction);

impl Default for LastDirection {
    fn default() -> Self {
        Self(Direction::Left)
    }
}

#[derive(Component)]
pub struct Food;

// point number
#[derive(Component, Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
