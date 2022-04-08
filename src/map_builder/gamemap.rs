use crate::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct GameMap {
    pub tiles: Vec<TileType>,
}

pub fn sprite_idx(x: i32, y: i32) -> usize {
    (y as usize * 16) + x as usize
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y as usize * 40) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 40 * 30];

    // Make the edges walls
    for x in 0..40 {
        map[map_idx(x, 0)] = TileType::Wall;
        map[map_idx(x, 29)] = TileType::Wall;
    }

    for y in 0..30 {
        map[map_idx(0, y)] = TileType::Wall;
        map[map_idx(39, y)] = TileType::Wall;
    }

    let mut rng = rand::thread_rng();

    for _i in 0..50 {
        let x = rng.gen_range(1..39);
        let y = rng.gen_range(1..29);
        let idx = map_idx(x, y);
        map[idx] = TileType::Wall;
    }

    map
}

pub fn draw_map(
    tiles: &Vec<TileType>,
    mut commands: Commands,
    texture_atlas_handle: &Handle<TextureAtlas>,
    sprite_specs: Res<SpriteSpecs>,
) {
    let mut x = -320.0 + sprite_specs.buffer;
    let mut y = -240.0 + sprite_specs.buffer;

    for tile in tiles.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                // Spawn the floor
                commands
                    .spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                        sprite: TextureAtlasSprite::new(sprite_idx(2, 3)),
                        ..Default::default()
                    })
                    .insert(Position { x, y });
            }
            TileType::Wall => {
                // Spawn the Wall
                commands
                    .spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                        sprite: TextureAtlasSprite::new(sprite_idx(0, 1)),
                        ..Default::default()
                    })
                    .insert(Blocking {})
                    .insert(Position { x, y });
            }
        }

        // Move the coordinates
        x += 16.0;
        if x > 320.0 - sprite_specs.size + sprite_specs.buffer {
            x = -320.0 + sprite_specs.buffer;
            y += 16.0;
        }
    }
}
