use crate::prelude::*;

mod map;
pub use map::*;

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        let map = Map::new();
        app.insert_resource(map).add_system_set(
            SystemSet::on_enter(TurnState::PlayerTurn).with_system(spawn_map_tiles),
        );
    }
}

pub struct MapsPlugin;
impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameMapPlugin);
    }
}
