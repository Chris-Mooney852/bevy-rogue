use bevy::prelude::*;

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
        .add_startup_system(setup)
        .add_system(player_movement)
        .run()
}

#[derive(Component)]
struct Player {
    x: f32,
    y: f32,
}

#[derive(PartialEq, Clone, Copy)]
enum TileType {
    Wall,
    Floor,
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    const MOVE_DISTANCE: f32 = 16.0;
    const SPRITE_BUFFER: f32 = 8.0;
    const X_BOUNDS: f32 = 320.0 - SPRITE_BUFFER;
    const Y_BOUNDS: f32 = 240.0 - SPRITE_BUFFER;

    for (mut player, mut trans) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Right) {
            player.x += MOVE_DISTANCE;
        }
        if keyboard_input.just_pressed(KeyCode::Left) {
            player.x -= MOVE_DISTANCE;
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            player.y += MOVE_DISTANCE;
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            player.y -= MOVE_DISTANCE;
        }

        // Apply movement deltas
        trans.translation.x += player.x;
        trans.translation.x = trans.translation.x.clamp(-X_BOUNDS, X_BOUNDS);
        trans.translation.y += player.y;
        trans.translation.y = trans.translation.y.clamp(-Y_BOUNDS, Y_BOUNDS);

        player.x = 0.0;
        player.y = 0.0;
    }
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 40 * 30];

    // Make the edges walls
    for x in (0..640).step_by(16) {
        map[map_idx(x, 0)] = TileType::Wall;
        map[map_idx(x, 39)] = TileType::Wall;
    }

    for y in (0..640).step_by(16) {
        map[map_idx(0, y)] = TileType::Wall;
        map[map_idx(29, y)] = TileType::Wall;
    }

    map
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add a 2D Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Spawn the player
    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -220.0, 0.0)),
            sprite: TextureAtlasSprite::new(sprite_idx(4, 8)),
            ..Default::default()
        })
        .insert(Player { x: 0.0, y: 0.0 });
}

pub fn sprite_idx(x: i32, y: i32) -> usize {
    (y as usize * 16) + x as usize
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y as usize * 40) + x as usize
}
