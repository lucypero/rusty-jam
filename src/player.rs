use bevy::{math::{vec2, vec3}, prelude::*};
use crate::collision::HitBoxEvent;

pub enum PlayerAction {
    Idle,
    Walk,
    Slash,
    Dash,
}

pub enum Facing {
    Up,
    Left,
    Down,
    Right
}

const MOVEMENT_SPEED :f32 = 3.;
const DASH_SPEED :f32 = 10.; // when dashing, vel *= dash_speed

pub struct Player {
    pub frame: u64,
    pub action: PlayerAction,
    pub facing: Facing,
    pub exp: u64,
    pub money: u64,
    pub vel: Vec2
}

impl Player {
    pub fn new() -> Self {
        Player {
            frame: 0,
            action: PlayerAction::Idle,
            facing: Facing::Right,
            exp: 0,
            money: 0,
            vel: vec2(0.,0.)
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
                let mut walking = false;
                // TODO(rukai): rewrite to use a velocity component. That way we keep momentum when e.g. stopping/slashing/knockback etc.

                if keyboard_input.pressed(KeyCode::W) &&
                   keyboard_input.pressed(KeyCode::A) {
                    player.vel = vec2(-1., 1.).normalize() * MOVEMENT_SPEED;
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Left;
                    walking = true;
                }
                else if keyboard_input.pressed(KeyCode::W) &&
                   keyboard_input.pressed(KeyCode::D) {

                    player.vel = vec2(1., 1.).normalize() * MOVEMENT_SPEED;
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Right;
                    walking = true;
                }
                else if keyboard_input.pressed(KeyCode::S) &&
                   keyboard_input.pressed(KeyCode::A) {

                    player.vel = vec2(-1., -1.).normalize() * MOVEMENT_SPEED;
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Left;
                    walking = true;
                }
                else if keyboard_input.pressed(KeyCode::S) &&
                   keyboard_input.pressed(KeyCode::D) {
                    player.vel = vec2(1., -1.).normalize() * MOVEMENT_SPEED;
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Right;
                    walking = true;
                }
                else if keyboard_input.pressed(KeyCode::W) {
                    // transform.translation.y += 3f32;
                    player.vel = vec2(0., MOVEMENT_SPEED);
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Up;
                    walking = true;
                }
                else if keyboard_input.pressed(KeyCode::S) {
                    // transform.translation.y -= 3f32;
                    player.vel = vec2(0., -MOVEMENT_SPEED);
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Down;
                    walking = true;
                }
                else if keyboard_input.pressed(KeyCode::A) {
                    // transform.translation.x -= 3f32;
                    player.vel = vec2(-MOVEMENT_SPEED, 0.);
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Left;
                    walking = true;
                }
                else if keyboard_input.pressed(KeyCode::D) {
                    // transform.translation.x += 3f32;
                    player.vel = vec2(MOVEMENT_SPEED, 0.);
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Right;
                    walking = true;
                }
                // TODO(rukai): activate slash on mouse click
                if keyboard_input.pressed(KeyCode::Q) {
                    player.set_action(PlayerAction::Slash);
                }
                if keyboard_input.pressed(KeyCode::Space) {
                    player.set_action(PlayerAction::Dash);
                }
                if !walking {
                    player.vel = vec2(0.,0.);
                }
            }
            PlayerAction::Slash => {
                // TODO(rukai): set velocity to go towards the mouse
                if player.frame == 2 {
                    // TODO(rukai): set hitbox position to be in front of the player towards the mouse
                    hitbox.send(HitBoxEvent {
                        position: transform.translation.truncate(),
                        size: Vec2::new(20.0, 20.0)
                    });
                }
                if player.frame > 20 {
                    player.set_action(PlayerAction::Idle);
                }
            }
            PlayerAction::Dash => {

                if player.frame == 1 {
                    player.vel *= DASH_SPEED;
                }

                println!("vel: {}", player.vel);

                if player.frame > 6 {
                    player.set_action(PlayerAction::Idle);
                }

            }
        }
        //apply vel and friction
        transform.translation += vec3(player.vel.x, player.vel.y, 0.0);

        player.frame += 1;
    }
}
