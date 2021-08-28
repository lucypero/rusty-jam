use bevy::prelude::*;
use crate::player::Player;
use crate::collision::{Hurtbox, Team};
use crate::skeleton::SkeletonBundle;

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

pub enum ShopkeeperAction {
    Idle,
    Walk,
    Damaged,
    SpawnMinions,
}

pub struct Shopkeeper {
    action: ShopkeeperAction,
    frame: u64,
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
    mut enemy_query: Query<(&mut Shopkeeper, &mut Hurtbox, &mut Transform), Without<Player>>,
) {
    if let Ok((_player, player_transform)) = player_query.single_mut() {
        for (mut shopkeeper, mut hurtbox, transform) in enemy_query.iter_mut() {
            if hurtbox.is_hit {
                shopkeeper.set_action(ShopkeeperAction::Damaged);
                hurtbox.is_hit = false;
            }

            let difference = player_transform.translation - transform.translation;
            match shopkeeper.action {
                ShopkeeperAction::Idle => {
                    if difference.length() < 100.0 {
                        shopkeeper.set_action(ShopkeeperAction::Walk);
                    }
                }
                ShopkeeperAction::Walk => {
                    hurtbox.vel = difference.truncate().normalize() * 1.5;
                    if shopkeeper.frame > 300 {
                        shopkeeper.set_action(ShopkeeperAction::SpawnMinions);
                    }
                }
                ShopkeeperAction::SpawnMinions => {
                    hurtbox.invincible = true;
                    // spawn minions perpindular to the player
                    let angle = transform.translation.angle_between(player_transform.translation); // TODO: angle_between docs say Vec3(0, 0, 0) is bad...?
                    println!("{}", angle);
                    if shopkeeper.frame % 2 == 0 {
                        hurtbox.vel = Vec2::new(angle.cos(), angle.sin()) * 20.0;
                    } else {
                        hurtbox.vel = Vec2::new(angle.cos(), angle.sin()) * -20.0;
                    }

                    if shopkeeper.frame == 50 {
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(angle.cos(), angle.sin()) * 500.0));
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(angle.cos(), angle.sin()) * 300.0));
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(angle.cos(), angle.sin()) * 100.0));
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(angle.cos(), angle.sin()) * -100.0));
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(angle.cos(), angle.sin()) * -300.0));
                        commands.spawn_bundle(SkeletonBundle::new(&mut materials, transform.translation.truncate() + Vec2::new(angle.cos(), angle.sin()) * -500.0));
                    }
                    if shopkeeper.frame > 60 {
                        hurtbox.invincible = false;
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
        }
    }
}
