use bevy::prelude::Component;

#[derive(Component, Clone, Debug)]
pub struct SnakePoints(pub Vec<Point>);

impl SnakePoints {
    pub fn new() -> Self {
        Self(vec![{ Point { x: 0_f32, y: 0_f32 } }])
    }

    pub fn head(&self) -> Point {
        self.0[0]
    }

    pub fn tail(&self) -> Point {
        self.0[self.0.len() - 1]
    }

    pub fn add_head(&mut self, point: Point) {
        self.0.insert(0, point);
    }

    pub fn remove_tail(&mut self) {
        self.0.pop();
    }

    pub fn push(&mut self, point: Point) {
        self.0.push(point);
    }
}

#[derive(Component)]
pub struct LastDirection(pub Direction);

// point number
#[derive(Component, Clone, Copy, Debug)]
pub struct Point {
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
