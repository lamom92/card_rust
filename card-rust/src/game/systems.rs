use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::game::background::Background;

// Update background size based on current window size
pub fn resize_background(
    mut resized: MessageReader<WindowResized>,
    windows: Query<&Window>,
    mut background_q: Query<&mut Sprite, With<Background>>,
) {
    if resized.is_empty() {
        return;
    }
    resized.clear();

    let window = windows
        .single()
        .expect("expected a primary window to exist");
    let new_size = Vec2::new(window.width() * 0.9, window.height() * 0.9);

    for mut sprite in &mut background_q {
        sprite.custom_size = Some(new_size);
    }
}
