use bevy::{prelude::Resource, time::Timer};

#[derive(Resource)]
pub struct StepTimer(pub Timer);
