use crate::prelude::*;

mod map;
pub use map::*;

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Map>();
    }
}

pub struct MapsPlugin;
impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameMapPlugin);
    }
}
