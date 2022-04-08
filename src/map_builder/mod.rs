use crate::prelude::*;

mod gamemap;
pub use gamemap::*;

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameMap>();
    }
}

pub struct MapsPlugin;
impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameMapPlugin);
    }
}
