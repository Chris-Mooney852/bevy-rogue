use bevy::prelude::*;
use rand::Rng;

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
        .init_resource::<Map>()
        .insert_resource(SpriteSpecs {
            size: 16.0,
            buffer: 8.0,
        })
        .add_startup_system(setup)
        .add_system(player_input)
        .run()
}

#[derive(Component)]
struct Player {}

#[derive(Component, Clone)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
struct Map {
    tiles: Vec<TileType>,
}

#[derive(Component)]
struct Blocking {}

#[derive(Default)]
struct SpriteSpecs {
    size: f32,
    buffer: f32,
}

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    sprite_specs: Res<SpriteSpecs>,
    mut query: Query<(&mut Position, &mut Transform), With<Player>>,
    blocking_entities: Query<&Position, (With<Blocking>, Without<Player>)>,
) {
    let move_distance: f32 = sprite_specs.size;
    let sprite_buffer: f32 = sprite_specs.buffer;
    let x_bounds: f32 = 320.0 - sprite_buffer;
    let y_bounds: f32 = 240.0 - sprite_buffer;
    let mut blocked = false;

    let (mut player_position, mut trans) = query.single_mut();
    let mut new_position = player_position.clone();

    let key = keyboard_input.get_just_pressed().next().cloned();

    if let Some(key) = key {
        match key {
            KeyCode::Right => new_position.x += move_distance,
            KeyCode::Left => new_position.x -= move_distance,
            KeyCode::Up => new_position.y += move_distance,
            KeyCode::Down => new_position.y -= move_distance,
            _ => return,
        };

        for blocking_position in blocking_entities.iter() {
            if new_position.x == blocking_position.x && new_position.y == blocking_position.y {
                blocked = true;
                break;
            }
        }

        if !blocked {
            // Apply movement deltas
            player_position.x = new_position.x.clamp(-x_bounds, x_bounds);
            player_position.y = new_position.y.clamp(-y_bounds, y_bounds);
            trans.translation.x = player_position.x;
            trans.translation.y = player_position.y;
        }
    }
}

fn new_map() -> Vec<TileType> {
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

fn draw_map(
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut map: ResMut<Map>,
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

pub fn map_idx(x: i32, y: i32) -> usize {
    (y as usize * 40) + x as usize
}
