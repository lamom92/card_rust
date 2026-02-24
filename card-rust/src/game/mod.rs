use bevy::prelude::*;

mod background;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, background::spawn_background)
            .add_systems(Update, systems::resize_background);
    }
}
