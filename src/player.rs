use bevy::prelude::*;

pub struct Player;

pub fn player_movement_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform)>) {
    if let Ok((_player, mut transform)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 3f32;
        }
        else if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 3f32;
        }
        else if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 3f32;
        }
        else if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 3f32;
        }
    }
}
