use bevy::prelude::{Component, Resource};

#[derive(Resource)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
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
