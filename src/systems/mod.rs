use crate::prelude::*;

mod player_input;

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_input::player_input);
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin);
    }
}
