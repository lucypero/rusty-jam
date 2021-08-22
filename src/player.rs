use bevy::prelude::*;

pub enum PlayerAction {
    Idle,
    Walk,
    Slash,
}

pub enum Facing {
    Up,
    Left,
    Down,
    Right
}

pub struct Player {
    pub frame: u64,
    pub action: PlayerAction,
    pub facing: Facing,
    pub exp: u64,
    pub money: u64,
}

impl Player {
    pub fn new() -> Self {
        Player {
            frame: 0,
            action: PlayerAction::Idle,
            facing: Facing::Right,
            exp: 0,
            money: 0,
        }
    }

    pub fn level(&self) -> u64 {
        self.exp / 100
    }

    pub fn set_action(&mut self, action: PlayerAction) {
        self.frame = 0;
        self.action = action;
    }
}

pub fn player_movement_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Player, &mut Transform)>) {
    if let Ok((mut player, mut transform)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 3f32;
            player.set_action(PlayerAction::Walk);
            player.facing = Facing::Up;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 3f32;
            player.set_action(PlayerAction::Walk);
            player.facing = Facing::Down;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 3f32;
            player.set_action(PlayerAction::Walk);
            player.facing = Facing::Left;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 3f32;
            player.set_action(PlayerAction::Walk);
            player.facing = Facing::Right;
        }

        if keyboard_input.pressed(KeyCode::Q) {
            player.set_action(PlayerAction::Slash);
        }

        player.frame += 1;
    }
}
