use bevy::prelude::*;
use crate::player::Player;
use crate::collision::{Hurtbox, Team, HitBoxEvent, CanHitTeam};
use crate::skeleton::SkeletonBundle;
use rand::seq::SliceRandom;

#[derive(Bundle)]
pub struct ShopkeeperBundle {
    shopkeeper: Shopkeeper,
    hurtbox: Hurtbox,
    #[bundle]
    sprite: SpriteBundle
}
impl ShopkeeperBundle {
    pub fn new(materials: &mut Assets<ColorMaterial>, position: Vec2) -> Self {
        Self {
            shopkeeper: Shopkeeper {
                action: ShopkeeperAction::Idle,
                frame: 0,
                frames_since_last_ability: 0,
            },
            hurtbox: Hurtbox {
                size: Vec2::new(30.0, 50.0),
                health: 50,
                team: Team::Enemy,
                is_hit: false,
                invincible: false,
                vel: Vec2::new(0.0, 0.0)
            },
            sprite: SpriteBundle {
                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                transform: Transform::from_translation(position.extend(0.0)),
                sprite: Sprite::new(Vec2::new(30.0, 50.0)),
                ..Default::default()
            }
        }
    }
}

#[derive(Clone)]
pub enum ShopkeeperAction {
    Idle,
    Walk,
    SpawnMinions,
    Blast,
    Damaged,
}

pub struct Shopkeeper {
    action: ShopkeeperAction,
    frame: u64,
    frames_since_last_ability: u64,
}

impl Shopkeeper {
    pub fn set_action(&mut self, action: ShopkeeperAction) {
        self.frame = 0;
        self.action = action;
    }
}

pub fn shopkeeper_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut shopkeeper_query: Query<(&mut Shopkeeper, &mut Hurtbox, &mut Transform), Without<Player>>,
    mut hitbox: EventWriter<HitBoxEvent>,
) {
    if let Ok((_player, player_transform)) = player_query.single_mut() {
        for (mut shopkeeper, mut hurtbox, transform) in shopkeeper_query.iter_mut() {
            if hurtbox.is_hit {
                shopkeeper.set_action(ShopkeeperAction::Damaged);
                hurtbox.is_hit = false;
            }

            let difference = player_transform.translation - transform.translation;
            match shopkeeper.action {
                ShopkeeperAction::Idle => {
                    if difference.length() < 100.0 {
                        shopkeeper.set_action(ShopkeeperAction::Walk);
                        shopkeeper.frames_since_last_ability = 0;
                    }
                }
                ShopkeeperAction::Walk => {
                    hurtbox.vel = difference.truncate().normalize() * 1.5;
                    if shopkeeper.frames_since_last_ability > 300 {
                        let action = [ShopkeeperAction::SpawnMinions, ShopkeeperAction::Blast].choose(&mut rand::thread_rng()).unwrap().clone();
                        shopkeeper.set_action(action);
                    }
                }
                ShopkeeperAction::Blast => {
                    hurtbox.invincible = true;
                    let angle = difference.angle_between(Vec3::new(1.0, 0.0, 0.0)); // TODO: angle_between docs say Vec3(0, 0, 0) is bad...?
                    if shopkeeper.frame < 8 {
                        hurtbox.vel = Vec2::new(angle.cos(), angle.sin()) * -10.0;
                    }

                    if shopkeeper.frame > 55 && shopkeeper.frame < 100 {
                        hitbox.send(HitBoxEvent {
                            position: transform.translation.truncate() + Vec2::new(angle.cos(), angle.sin()) * 150.0,
                            size: Vec2::new(300.0, 300.0),
                            damage: 5,
                            knockback: 70.0,
                            can_hit: CanHitTeam::Player,
                        });
                    }
                    if shopkeeper.frame > 105 {
                        hurtbox.invincible = false;
                        shopkeeper.frames_since_last_ability = 0;
                        shopkeeper.set_action(ShopkeeperAction::Walk);
                    }
                }
                // spawn minions perpindular to the player
                ShopkeeperAction::SpawnMinions => {
                    hurtbox.invincible = true;
                    let angle = difference.angle_between(Vec3::new(1.0, 0.0, 0.0)); // TODO: angle_between docs say Vec3(0, 0, 0) is bad...?
                    let perpindicular = angle + std::f32::consts::FRAC_PI_2;
                    if shopkeeper.frame % 2 == 0 {
                        hurtbox.vel = Vec2::new(perpindicular.cos(), perpindicular.sin()) * 20.0;
                    } else {
                        hurtbox.vel = Vec2::new(perpindicular.cos(), perpindicular.sin()) * -20.0;
                    }

                    if shopkeeper.frame == 50 {
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(perpindicular.cos(), perpindicular.sin()) * 500.0));
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(perpindicular.cos(), perpindicular.sin()) * 300.0));
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(perpindicular.cos(), perpindicular.sin()) * 100.0));
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(perpindicular.cos(), perpindicular.sin()) * -100.0));
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(perpindicular.cos(), perpindicular.sin()) * -300.0));
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(perpindicular.cos(), perpindicular.sin()) * -500.0));
                    }
                    if shopkeeper.frame > 60 {
                        hurtbox.invincible = false;
                        shopkeeper.frames_since_last_ability = 0;
                        shopkeeper.set_action(ShopkeeperAction::Walk);
                    }
                }
                ShopkeeperAction::Damaged => {
                    if shopkeeper.frame > 10 {
                        hurtbox.invincible = false;
                        shopkeeper.set_action(ShopkeeperAction::Walk);
                    }
                }
            }

            shopkeeper.frame += 1;
            shopkeeper.frames_since_last_ability += 1;
        }
    }
}
