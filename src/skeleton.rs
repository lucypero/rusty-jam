use bevy::prelude::*;
use crate::player::Player;
use crate::collision::{Hurtbox, Team};

#[derive(Bundle)]
pub struct SkeletonBundle {
    skeleton: Skeleton,
    hurtbox: Hurtbox,
    #[bundle]
    sprite: SpriteBundle
}
impl SkeletonBundle {
    pub fn new(materials: &mut Assets<ColorMaterial>, position: Vec2) -> Self {
        Self {
            skeleton: Skeleton {
                action: SkeletonAction::Walk,
                frame: 0,
            },
            hurtbox: Hurtbox {
                size: Vec2::new(60.0, 100.0),
                health: 10,
                team: Team::Enemy,
                is_hit: false,
                invincible: false,
                vel: Vec2::new(0.0, 0.0)
            },
            sprite: SpriteBundle {
                material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                transform: Transform::from_translation(position.extend(0.0)),
                sprite: Sprite::new(Vec2::new(60.0, 100.0)),
                ..Default::default()
            }
        }
    }
}

pub enum SkeletonAction {
    Walk,
    Damaged
}

pub struct Skeleton {
    action: SkeletonAction,
    frame: u64,
}

impl Skeleton {
    pub fn set_action(&mut self, action: SkeletonAction) {
        self.frame = 0;
        self.action = action;
    }
}

pub fn skeleton_system(
    mut player_query: Query<(&mut Player, &Transform)>,
    mut enemy_query: Query<(&mut Skeleton, &mut Hurtbox, &mut Transform), Without<Player>>,
) {
    if let Ok((_player, player_transform)) = player_query.single_mut() {
        for (mut skeleton, mut hurtbox, mut en_transform) in enemy_query.iter_mut() {
            if hurtbox.is_hit {
                skeleton.set_action(SkeletonAction::Damaged);
                hurtbox.is_hit = false;
            }

            match &skeleton.action {
                SkeletonAction::Walk => {
                    hurtbox.invincible = false;
                    let mut vec = player_transform.translation - en_transform.translation;
                    vec = vec.normalize() * 1.5;
                    en_transform.translation += vec;
                }
                SkeletonAction::Damaged => {
                    if skeleton.frame > 15 {
                        skeleton.set_action(SkeletonAction::Walk);
                    }
                }
            }

            skeleton.frame += 1;
        }
    }
}
