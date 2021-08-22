use bevy::prelude::*;
use crate::collision::HitBoxEvent;

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

pub fn player_movement_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Player, &mut Transform)>, mut hitbox: EventWriter<HitBoxEvent>) {
    if let Ok((mut player, mut transform)) = query.single_mut() {
        match player.action {
            PlayerAction::Idle | PlayerAction::Walk => {
                // TODO: rewrite to use a velocity component. That way we keep momentum when e.g. stopping/slashing/knockback etc.
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
                // TODO: activate slash on mouse click
                if keyboard_input.pressed(KeyCode::Q) {
                    player.set_action(PlayerAction::Slash);
                }
            }
            PlayerAction::Slash => {
                // TODO: set velocity to go towards the mouse
                if player.frame == 2 {
                    // TODO: set hitbox position to be in front of the player towards the mouse
                    hitbox.send(HitBoxEvent {
                        position: transform.translation.truncate(),
                        size: Vec2::new(20.0, 20.0)
                    });
                }
                if player.frame > 20 {
                    player.set_action(PlayerAction::Idle);
                }
            }
        }

        player.frame += 1;
    }
}
