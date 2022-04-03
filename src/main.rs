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

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Position, &mut Transform), With<Player>>,
    blocking_entities: Query<&Position, (With<Blocking>, Without<Player>)>,
) {
    const MOVE_DISTANCE: f32 = 16.0;
    const SPRITE_BUFFER: f32 = 8.0;
    const X_BOUNDS: f32 = 320.0 - SPRITE_BUFFER;
    const Y_BOUNDS: f32 = 240.0 - SPRITE_BUFFER;
    let mut blocked = false;

    let (mut player_position, mut trans) = query.single_mut();
    let mut new_position = player_position.clone();

    let key = keyboard_input.get_just_pressed().next().cloned();

    if let Some(key) = key {
        match key {
            KeyCode::Right => new_position.x += MOVE_DISTANCE,
            KeyCode::Left => new_position.x -= MOVE_DISTANCE,
            KeyCode::Up => new_position.y += MOVE_DISTANCE,
            KeyCode::Down => new_position.y -= MOVE_DISTANCE,
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
            player_position.x = new_position.x.clamp(-X_BOUNDS, X_BOUNDS);
            player_position.y = new_position.y.clamp(-Y_BOUNDS, Y_BOUNDS);
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
) {
    const SPRITE_SIZE: f32 = 16.0;
    const SPRITE_BUFFER: f32 = 8.0;
    let mut x = -320.0 + SPRITE_BUFFER;
    let mut y = -240.0 + SPRITE_BUFFER;

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
        if x > 320.0 - SPRITE_SIZE + SPRITE_BUFFER {
            x = -320.0 + SPRITE_BUFFER;
            y += 16.0;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut map: ResMut<Map>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    map.tiles = new_map();

    // Add a 2D Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Spawn the player
    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(8.0, 8.0, 0.1)),
            sprite: TextureAtlasSprite::new(sprite_idx(4, 8)),
            ..Default::default()
        })
        .insert(Player {})
        .insert(Position { x: 8.0, y: 8.0 });

    draw_map(&map.tiles, commands, &texture_atlas_handle);
}

pub fn sprite_idx(x: i32, y: i32) -> usize {
    (y as usize * 16) + x as usize
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y as usize * 40) + x as usize
}
