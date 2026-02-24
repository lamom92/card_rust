use bevy::prelude::*;

// Marker component for the background entity
#[derive(Component)]
pub struct Background;

// Spawn camera + background on startup
pub fn spawn_background(mut commands: Commands, windows: Query<&Window>) {
    commands.spawn(Camera2d);

    let window = windows
        .single()
        .expect("expected a primary window to exist");
    let size = Vec2::new(window.width() * 0.9, window.height() * 0.9);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.15, 0.4, 0.2), size),
        Transform::default(),
        Background,
    ));
}
