use bevy::prelude::*;

mod table;
mod systems;

pub struct GamePlugin;

impl plugin for GamePlugin {
    fn build(&self, app:: &mut App) {
        app.add_systems(Startup, table::spaw_table)
            .add_systems(Update, systems::rezise_table);
    }
}
