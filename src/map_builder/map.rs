use crate::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        const TILES_X: i32 = SCREEN_WIDTH as i32 / SPRITE_SIZE as i32;
        const TILES_Y: i32 = SCREEN_HEIGHT as i32 / SPRITE_SIZE as i32;

        let mut tiles = vec![TileType::Floor; (TILES_X * TILES_Y) as usize];

        // Make the edges walls
        for x in 0..TILES_X {
            tiles[map_idx(x, 0)] = TileType::Wall;
            tiles[map_idx(x, TILES_Y - 1)] = TileType::Wall;
        }

        for y in 0..TILES_Y {
            tiles[map_idx(0, y)] = TileType::Wall;
            tiles[map_idx(TILES_X - 1, y)] = TileType::Wall;
        }

        let mut rng = rand::thread_rng();

        for _i in 0..200 {
            let x = rng.gen_range(1..TILES_X - 1);
            let y = rng.gen_range(1..TILES_Y - 1);
            let idx = map_idx(x, y);
            tiles[idx] = TileType::Wall;
        }

        Self { tiles }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    const TILES_X: usize = SCREEN_WIDTH as usize / SPRITE_SIZE as usize;
    (y as usize * TILES_X) + x as usize
}

pub fn spawn_map_tiles(
    map: Res<Map>,
    mut commands: Commands,
    texture_atlas_handle: Res<Handle<TextureAtlas>>,
) {
    const HALF_WIDTH: f32 = SCREEN_WIDTH / 2.0;
    const HALF_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;
    let mut x = -HALF_WIDTH + SPRITE_BUFFER;
    let mut y = -HALF_HEIGHT + SPRITE_BUFFER;

    for tile in map.tiles.iter() {
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
        x += SPRITE_SIZE;
        if x > HALF_WIDTH - SPRITE_SIZE + SPRITE_BUFFER {
            x = -HALF_WIDTH + SPRITE_BUFFER;
            y += SPRITE_SIZE;
        }
    }
}

pub fn sprite_idx(x: i32, y: i32) -> usize {
    (y as usize * SPRITE_SIZE as usize) + x as usize
}
