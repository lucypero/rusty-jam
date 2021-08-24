use bevy::{math::{vec2, vec3}, prelude::*};
use crate::{DAMAGED_INVINSIBILITY_FRAMES, DASH_COOLDOWN_TIME, DASH_DURATION, DASH_SPEED, MOVEMENT_SPEED, collision::HitBoxEvent};

pub enum PlayerAction {
    Idle,
    Walk,
    Slash,
    Dash,
    Damaged
}

pub enum Facing {
    Up,
    Left,
    Down,
    Right
}

pub struct Player {
    pub frame: u64,
    pub frame_since_last_cooldown: u64,
    pub action: PlayerAction,
    pub facing: Facing,
    pub hp: i32,
    pub exp: u64,
    pub money: u64,
    pub vel: Vec2,
    pub invinsible: bool,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hp: 1000,
            frame: 0,
            frame_since_last_cooldown: 0,
            action: PlayerAction::Idle,
            facing: Facing::Right,
            exp: 0,
            money: 0,
            vel: vec2(0.,0.),
            invinsible: false,
        }
    }

    pub fn level(&self) -> u64 {
        self.exp / 100
    }

    pub fn set_action(&mut self, action: PlayerAction) {
        self.frame = 0;
        self.action = action;
    }

    pub fn take_damage(&mut self, t: &mut Transform, damage: i32, recoil_vec: Vec2) {
        self.set_action(PlayerAction::Damaged);
        self.vel = recoil_vec;
        self.invinsible = true;
    }
}

pub fn player_movement_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Player, &mut Transform)>, mut hitbox: EventWriter<HitBoxEvent>) {
    if let Ok((mut player, mut transform)) = query.single_mut() {
        match player.action {
            PlayerAction::Idle | PlayerAction::Walk => {
                let mut walking = false;

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
                if keyboard_input.just_pressed(KeyCode::Space) {
                    if player.frame_since_last_cooldown > DASH_COOLDOWN_TIME as u64{
                        player.set_action(PlayerAction::Dash);
                    }
                }
                if !walking {
                    player.vel = vec2(0.,0.);
                }
            }
            PlayerAction::Slash => {
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

                player.frame_since_last_cooldown = 0;
                player.invinsible = true;

                if player.frame == 1 {
                    player.vel *= DASH_SPEED;
                }

                if player.frame > DASH_DURATION as u64{
                    player.invinsible = false;
                    player.set_action(PlayerAction::Idle);
                }

            },
            PlayerAction::Damaged => {
                if player.frame > DAMAGED_INVINSIBILITY_FRAMES as u64 {
                    player.invinsible = false;
                    player.set_action(PlayerAction::Idle);
                }
            }
        }
        //apply vel and friction
        transform.translation += vec3(player.vel.x, player.vel.y, 0.0);

        player.frame += 1;
        player.frame_since_last_cooldown += 1;
    }
}
