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
        .add_system(player_movement)
        .run()
}

#[derive(Component)]
struct Player {}

#[derive(Component)]
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

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Position, &mut Transform)>,
) {
    const MOVE_DISTANCE: f32 = 16.0;
    const SPRITE_BUFFER: f32 = 8.0;
    const X_BOUNDS: f32 = 320.0 - SPRITE_BUFFER;
    const Y_BOUNDS: f32 = 240.0 - SPRITE_BUFFER;

    for (mut _player, mut position, mut trans) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Right) {
            position.x += MOVE_DISTANCE;
        }
        if keyboard_input.just_pressed(KeyCode::Left) {
            position.x -= MOVE_DISTANCE;
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            position.y += MOVE_DISTANCE;
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            position.y -= MOVE_DISTANCE;
        }

        // Apply movement deltas
        trans.translation.x += position.x;
        trans.translation.x = trans.translation.x.clamp(-X_BOUNDS, X_BOUNDS);
        trans.translation.y += position.y;
        trans.translation.y = trans.translation.y.clamp(-Y_BOUNDS, Y_BOUNDS);

        position.x = 0.0;
        position.y = 0.0;
    }
}

fn try_move_player(
    dx: f32,
    dy: f32,
    map: Res<Map>,
    mut query: Query<(&mut Player, &mut Position, &mut Transform)>,
) {

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
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
            sprite: TextureAtlasSprite::new(sprite_idx(4, 8)),
            ..Default::default()
        })
        .insert(Player {})
        .insert(Position { x: 10.0, y: 10.0 });

    draw_map(&map.tiles, commands, &texture_atlas_handle);
}

pub fn sprite_idx(x: i32, y: i32) -> usize {
    (y as usize * 16) + x as usize
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y as usize * 40) + x as usize
}

pub fn xy_to_map_idx(x: f32, y: f32) -> usize{
    const SPRITE_SIZE: f32 = 16.0;
    const SPRITE_BUFFER: f32 = 8.0;

    let buffed_x = x - SPRITE_SIZE + SPRITE_BUFFER;
    let buffed_y = y - SPRITE_SIZE + SPRITE_BUFFER;

    map_idx(buffed_x as i32, buffed_y as i32)
}
