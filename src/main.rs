mod components;
mod map_builder;
mod resources;
mod systems;

mod prelude {
    pub use crate::components::*;
    pub use crate::map_builder::*;
    pub use crate::resources::*;
    pub use crate::systems::*;
    pub use bevy::prelude::*;
    pub use rand::Rng;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Rogue".to_string(),
            width: 640.0,
            height: 480.0,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SystemsPlugin)
        .add_plugin(MapsPlugin)
        .insert_resource(SpriteSpecs {
            size: 16.0,
            buffer: 8.0,
        })
        .add_startup_system(setup)
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut map: ResMut<GameMap>,
    sprite_specs: Res<SpriteSpecs>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(sprite_specs.size, sprite_specs.size),
        sprite_specs.size as usize,
        sprite_specs.size as usize,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    map.tiles = new_map();

    // Add a 2D Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Spawn the player
    let player_x = sprite_specs.size - sprite_specs.buffer;
    let player_y = sprite_specs.size - sprite_specs.buffer;

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(player_x, player_y, 0.1)),
            sprite: TextureAtlasSprite::new(sprite_idx(4, 8)),
            ..Default::default()
        })
        .insert(Player {})
        .insert(Position {
            x: player_x,
            y: player_y,
        });

    draw_map(&map.tiles, commands, &texture_atlas_handle, sprite_specs);
}

pub fn sprite_idx(x: i32, y: i32) -> usize {
    (y as usize * 16) + x as usize
}
