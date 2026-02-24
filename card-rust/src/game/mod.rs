use bevy::prelude::*;

mod table;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, table::spawn_table)
            .add_systems(Update, systems::resize_table);
    }
}
