use crate::prelude::*;

pub fn player_input(
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
