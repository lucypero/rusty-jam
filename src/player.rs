use bevy::{math::{vec2, vec3}, prelude::*};
use crate::{DAMAGED_INVINCIBILITY_FRAMES, DASH_COOLDOWN_TIME, DASH_DURATION, DASH_SPEED, MOVEMENT_SPEED, collision::HitBoxEvent};
use crate::mouse::MouseState;

pub enum PlayerAction {
    Idle,
    Walk,
    Slash { angle: f32 },
    Dash { angle: f32 },
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
    pub invincible: bool,
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
            invincible: false,
        }
    }

    pub fn level(&self) -> u64 {
        self.exp / 100
    }

    pub fn set_action(&mut self, action: PlayerAction) {
        self.frame = 0;
        self.action = action;
    }

    pub fn take_damage(&mut self, recoil_vec: Vec2) {
        self.set_action(PlayerAction::Damaged);
        self.vel = recoil_vec;
        self.invincible = true;
    }
}

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse: Res<MouseState>,
    mut query: Query<(&mut Player, &mut Transform)>,
    mut hitbox: EventWriter<HitBoxEvent>,
) {
    if let Ok((mut player, mut transform)) = query.single_mut() {
        match player.action {
            PlayerAction::Idle | PlayerAction::Walk => {
                if keyboard_input.pressed(KeyCode::W) &&
                   keyboard_input.pressed(KeyCode::A) {
                    player.vel = vec2(-1., 1.).normalize() * MOVEMENT_SPEED;
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Left;
                }
                else if keyboard_input.pressed(KeyCode::W) &&
                   keyboard_input.pressed(KeyCode::D) {

                    player.vel = vec2(1., 1.).normalize() * MOVEMENT_SPEED;
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Right;
                }
                else if keyboard_input.pressed(KeyCode::S) &&
                   keyboard_input.pressed(KeyCode::A) {

                    player.vel = vec2(-1., -1.).normalize() * MOVEMENT_SPEED;
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Left;
                }
                else if keyboard_input.pressed(KeyCode::S) &&
                   keyboard_input.pressed(KeyCode::D) {
                    player.vel = vec2(1., -1.).normalize() * MOVEMENT_SPEED;
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Right;
                }
                else if keyboard_input.pressed(KeyCode::W) {
                    player.vel = vec2(0., MOVEMENT_SPEED);
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Up;
                }
                else if keyboard_input.pressed(KeyCode::S) {
                    player.vel = vec2(0., -MOVEMENT_SPEED);
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Down;
                }
                else if keyboard_input.pressed(KeyCode::A) {
                    player.vel = vec2(-MOVEMENT_SPEED, 0.);
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Left;
                }
                else if keyboard_input.pressed(KeyCode::D) {
                    player.vel = vec2(MOVEMENT_SPEED, 0.);
                    player.set_action(PlayerAction::Walk);
                    player.facing = Facing::Right;
                }
                if mouse_button_input.just_pressed(MouseButton::Left) {
                    if let Some(angle) = mouse.angle_from_location_to_mouse(transform.translation.truncate()) {
                        player.set_action(PlayerAction::Slash { angle });
                        player.vel = vec2(angle.cos() * 4.0, angle.sin() * 4.0);
                    }
                }
                if keyboard_input.just_pressed(KeyCode::Space) {
                    if player.frame_since_last_cooldown > DASH_COOLDOWN_TIME as u64 {
                        if let Some(angle) = mouse.angle_from_location_to_mouse(transform.translation.truncate()) {
                            player.set_action(PlayerAction::Dash { angle });
                        }
                    }
                }
            }
            PlayerAction::Slash { angle } => {
                if player.frame < 10 {
                    player.vel = vec2(angle.cos() * 3.0, angle.sin() * 3.0);
                    hitbox.send(HitBoxEvent {
                        position: transform.translation.truncate() + Vec2::new(angle.cos(), angle.sin()) * 40.0,
                        size: Vec2::new(30.0, 30.0)
                    });
                }
                if player.frame > 17 {
                    player.set_action(PlayerAction::Idle);
                }
            }
            PlayerAction::Dash { angle } => {
                player.frame_since_last_cooldown = 0;
                player.invincible = true;

                if player.frame < 4 {
                    player.vel = Vec2::new(angle.cos(), angle.sin()) * DASH_SPEED;
                }

                if player.frame > DASH_DURATION as u64{
                    player.invincible = false;
                    player.set_action(PlayerAction::Idle);
                }
            },
            PlayerAction::Damaged => {
                if player.frame > DAMAGED_INVINCIBILITY_FRAMES as u64 {
                    player.invincible = false;
                    player.set_action(PlayerAction::Idle);
                }
            }
        }
        //apply vel and friction
        player.vel *= 0.8;
        transform.translation += vec3(player.vel.x, player.vel.y, 0.0);

        player.frame += 1;
        player.frame_since_last_cooldown += 1;
    }
}
