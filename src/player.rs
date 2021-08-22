use bevy::prelude::*;

pub struct Player {
    pub exp: u64,
    pub money: u64,
}

impl Player {
    pub fn new() -> Self {
        Player {
            exp: 0,
            money: 0,
        }
    }

    pub fn level(&self) -> u64 {
        self.exp / 100
    }
}

pub fn player_movement_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform)>) {
    if let Ok((_player, mut transform)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 3f32;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 3f32;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 3f32;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 3f32;
        }
    }
}
