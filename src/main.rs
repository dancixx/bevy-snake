use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::Color;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Snake Game".to_string(),
                width: 800.0,
                height: 600.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}

struct WindowWidth(f32);
struct WindowHeight(f32);

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn setup(mut commands: Commands) {
    let window_width = WindowWidth(800.0);
    let window_height = WindowHeight(600.0);

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GREEN),
        },
        ..Default::default()
    });

    let random_x = rand::random::<f32>() * 800.0;
    let random_y = rand::random::<f32>() * 600.0;

    println!("Random x: {}", random_x);
    println!("Random y: {}", random_y);

    commands.spawn(Position {
        x: random_x,
        y: random_y,
    });

    let text_style = TextStyle {
        font_size: 40.0,
        color: Color::BLACK,
        ..Default::default()
    };
    let text_position = Position {
        x: random_x,
        y: random_y,
    };
    commands.spawn(TextBundle {
        text: Text::from_section("+", text_style),
        style: Style {
            position: UiRect {
                left: Val::Px(100f32),
                right: Val::Px(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}
