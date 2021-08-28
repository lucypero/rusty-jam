use bevy::{math::{vec2, vec3}, prelude::*};
use crate::{DAMAGED_INVINCIBILITY_FRAMES, DASH_COOLDOWN_TIME, DASH_DURATION, DASH_SPEED, MOVEMENT_SPEED, collision::HitBoxEvent, Hurtbox};
use crate::mouse::MouseState;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    hurtbox: Hurtbox,
    #[bundle]
    sprite: SpriteSheetBundle
}
impl PlayerBundle {
    pub fn new(texture_atlas: Handle<TextureAtlas>) -> Self {
        Self {
            player: Player::new(),
            hurtbox: Hurtbox {
                size: Vec2::new(30.0, 50.0),
                health: 200,
            },
            sprite: SpriteSheetBundle {
                transform: Transform::from_scale(Vec3::splat(3.0)),
                texture_atlas: texture_atlas,
                ..Default::default()
            }
        }
    }
}

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

    pub fn set_facing_by_angle(&mut self, angle: f32) {
        let (sin, cos) = angle.sin_cos();
        self.facing = if sin.abs() > cos.abs() {
            if sin.is_sign_positive() {
                Facing::Up
            } else {
                Facing::Down
            }
        } else {
            if cos.is_sign_positive() {
                Facing::Right
            } else {
                Facing::Left
            }
        }
    }

    pub fn take_damage(&mut self, recoil_vec: Vec2) {
        self.set_action(PlayerAction::Damaged);
        self.vel = recoil_vec;
        self.invincible = true;
    }

    pub fn check_enter_walk(
        &mut self,
        keyboard_input: &Input<KeyCode>,
    ) -> bool {
        if
            keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::S) ||
            keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D)
        {
            self.set_action(PlayerAction::Walk);
            false
        } else {
            true
        }
    }

    pub fn check_leave_walk(
        &mut self,
        keyboard_input: &Input<KeyCode>,
    ) -> bool {
        if
            !keyboard_input.pressed(KeyCode::W) && !keyboard_input.pressed(KeyCode::S) &&
            !keyboard_input.pressed(KeyCode::A) && !keyboard_input.pressed(KeyCode::D)
        {
            self.set_action(PlayerAction::Idle);
            false
        } else {
            true
        }
    }

    pub fn check_actions(
        &mut self,
        keyboard_input: &Input<KeyCode>,
        mouse: &MouseState,
        mouse_button_input: &Input<MouseButton>,
        transform: &mut Transform
    ) -> bool {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            if let Some(angle) = mouse.angle_from_location_to_mouse(transform.translation.truncate()) {
                self.set_action(PlayerAction::Slash { angle });
                self.vel = vec2(angle.cos() * 4.0, angle.sin() * 4.0);
                self.set_facing_by_angle(angle);
            }
            false
        }
        else if keyboard_input.just_pressed(KeyCode::Space) && self.frame_since_last_cooldown > DASH_COOLDOWN_TIME as u64 {
            if let Some(angle) = mouse.angle_from_location_to_mouse(transform.translation.truncate()) {
                self.set_action(PlayerAction::Dash { angle });
                self.set_facing_by_angle(angle);
            }
            false
        }
        else {
            true
        }
    }

    pub fn idle_action(
        &mut self,
        keyboard_input: &Input<KeyCode>,
        mouse: &MouseState,
        mouse_button_input: &Input<MouseButton>,
        transform: &mut Transform
    ) {
        if self.check_actions(&keyboard_input, &mouse, &mouse_button_input, transform) && self.check_enter_walk(&keyboard_input) {
            if self.frame > 90 {
                self.set_action(PlayerAction::Idle);
            }
        }
    }

    pub fn walk_action(
        &mut self,
        keyboard_input: &Input<KeyCode>,
        mouse: &MouseState,
        mouse_button_input: &Input<MouseButton>,
        transform: &mut Transform
    ) {
        if self.check_actions(&keyboard_input, &mouse, &mouse_button_input, transform) && self.check_leave_walk(&keyboard_input) {
            if keyboard_input.pressed(KeyCode::W) &&
                keyboard_input.pressed(KeyCode::A) {
                self.vel = vec2(-1., 1.).normalize() * MOVEMENT_SPEED;
                self.facing = Facing::Left;
            }
            else if keyboard_input.pressed(KeyCode::W) &&
                keyboard_input.pressed(KeyCode::D) {
                self.vel = vec2(1., 1.).normalize() * MOVEMENT_SPEED;
                self.facing = Facing::Right;
            }
            else if keyboard_input.pressed(KeyCode::S) &&
                keyboard_input.pressed(KeyCode::A) {
                self.vel = vec2(-1., -1.).normalize() * MOVEMENT_SPEED;
                self.facing = Facing::Left;
            }
            else if keyboard_input.pressed(KeyCode::S) &&
                keyboard_input.pressed(KeyCode::D) {
                self.vel = vec2(1., -1.).normalize() * MOVEMENT_SPEED;
                self.facing = Facing::Right;
            }
            else if keyboard_input.pressed(KeyCode::W) {
                self.vel = vec2(0., MOVEMENT_SPEED);
                self.facing = Facing::Up;
            }
            else if keyboard_input.pressed(KeyCode::S) {
                self.vel = vec2(0., -MOVEMENT_SPEED);
                self.facing = Facing::Down;
            }
            else if keyboard_input.pressed(KeyCode::A) {
                self.vel = vec2(-MOVEMENT_SPEED, 0.);
                self.facing = Facing::Left;
            }
            else if keyboard_input.pressed(KeyCode::D) {
                self.vel = vec2(MOVEMENT_SPEED, 0.);
                self.facing = Facing::Right;
            }

            if self.frame > 30 {
                self.set_action(PlayerAction::Walk);
            }
        }
    }

    pub fn slash_action(
        &mut self,
        angle: f32,
        keyboard_input: &Input<KeyCode>,
        mouse: &MouseState,
        mouse_button_input: &Input<MouseButton>,
        transform: &mut Transform,
        hitbox: &mut EventWriter<HitBoxEvent>,
    ) {
        if self.frame < 10 {
            self.vel = vec2(angle.cos() * 8.0, angle.sin() * 8.0);
            hitbox.send(HitBoxEvent {
                position: transform.translation.truncate() + Vec2::new(angle.cos(), angle.sin()) * 40.0,
                size: Vec2::new(30.0, 30.0)
            });
        }
        if self.frame > 17 {
            self.set_action(PlayerAction::Idle);
            self.idle_action(&keyboard_input, &mouse, &mouse_button_input, transform);
        }
    }

    pub fn dash_action(
        &mut self,
        angle: f32,
        keyboard_input: &Input<KeyCode>,
        mouse: &MouseState,
        mouse_button_input: &Input<MouseButton>,
        transform: &mut Transform,
    ) {
        self.frame_since_last_cooldown = 0;
        self.invincible = true;

        if self.frame < 4 {
            self.vel = Vec2::new(angle.cos(), angle.sin()) * DASH_SPEED;
        }

        if self.frame > DASH_DURATION as u64{
            self.invincible = false;
            self.set_action(PlayerAction::Idle);
            self.idle_action(&keyboard_input, &mouse, &mouse_button_input, transform);
        }
    }

    pub fn damaged_action(
        &mut self,
        keyboard_input: &Input<KeyCode>,
        mouse: &MouseState,
        mouse_button_input: &Input<MouseButton>,
        transform: &mut Transform,
    ) {
        if self.frame > DAMAGED_INVINCIBILITY_FRAMES as u64 {
            self.invincible = false;
            self.set_action(PlayerAction::Idle);
            self.idle_action(&keyboard_input, &mouse, &mouse_button_input, transform);
        }
    }
}

pub fn player_system(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse: Res<MouseState>,
    mut query: Query<(&mut Player, &mut Transform, &mut TextureAtlasSprite)>,
    mut hitbox: EventWriter<HitBoxEvent>,
) {
    if let Ok((mut player, mut transform, mut sprite)) = query.single_mut() {
        match player.action {
            PlayerAction::Idle => {
                player.idle_action(&keyboard_input, &mouse, &mouse_button_input, &mut transform);
            }
            PlayerAction::Walk => {
                player.walk_action(&keyboard_input, &mouse, &mouse_button_input, &mut transform);
            }
            PlayerAction::Slash { angle } => {
                player.slash_action(angle, &keyboard_input, &mouse, &mouse_button_input, &mut transform, &mut hitbox);
            }
            PlayerAction::Dash { angle } => {
                player.dash_action(angle, &keyboard_input, &mouse, &mouse_button_input, &mut transform);
            },
            PlayerAction::Damaged => {
                player.damaged_action(&keyboard_input, &mouse, &mouse_button_input, &mut transform);
            }
        }

        match (&player.action, &player.facing) {
            (PlayerAction::Idle, facing) => {
                let frame = if player.frame < 45 {
                    0
                } else {
                    1
                };
                set_sprite_index(&mut sprite, frame, 1);
                set_sprite_flip_from_facing(&mut sprite, facing);
            }
            (PlayerAction::Walk, facing) => {
                let frame = if player.frame < 15 {
                    0
                } else {
                    1
                };
                set_sprite_index(&mut sprite, frame, 0);
                set_sprite_flip_from_facing(&mut sprite, facing);
            }
            (PlayerAction::Slash { .. }, facing) => {
                sprite.index = 0;
                set_sprite_index(&mut sprite, 0, 0);
                set_sprite_flip_from_facing(&mut sprite, facing);
            }
            (PlayerAction::Dash { .. }, facing) => {
                sprite.index = 0;
                set_sprite_index(&mut sprite, 0, 0);
                set_sprite_flip_from_facing(&mut sprite, facing);
            }
            (PlayerAction::Damaged, facing) => {
                sprite.index = 0;
                set_sprite_index(&mut sprite, 0, 0);
                set_sprite_flip_from_facing(&mut sprite, facing);
            }
        }

        //apply vel and friction
        player.vel *= 0.8;
        transform.translation += vec3(player.vel.x, player.vel.y, 0.0);

        player.frame += 1;
        player.frame_since_last_cooldown += 1;
    }
}

fn set_sprite_index(sprite: &mut TextureAtlasSprite, x: u32, y: u32) {
    sprite.index = y * crate::PLAYER_SPRITE_ROWS + x;
}

fn set_sprite_flip_from_facing(sprite: &mut TextureAtlasSprite, facing: &Facing) {
    sprite.flip_x = match facing {
        Facing::Up => false,
        Facing::Down => false,
        Facing::Right => false,
        Facing::Left => true,
    }
}
