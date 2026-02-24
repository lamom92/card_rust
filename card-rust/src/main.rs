use bevy::prelude::*;
use bevy::window::WindowResolution;

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Card games".to_string(),
                resolution: WindowResolution::new(900, 600),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(game::GamePlugin)
        .run();
}
