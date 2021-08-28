use bevy::prelude::*;
use crate::player::Player;
use crate::Hurtbox;

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
                health: 1,
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
    // SomeAttack,
    // Damaged
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
    mut player_query: Query<(&mut Player, &Transform)>,
    mut enemy_query: Query<(&mut Shopkeeper, &mut Transform), Without<Player>>,
) {
    if let Ok((_player, player_transform)) = player_query.single_mut() {
        for (mut shopkeeper, mut transform) in enemy_query.iter_mut() {

            let difference = player_transform.translation - transform.translation;
            match shopkeeper.action {
                ShopkeeperAction::Idle => {
                    if difference.length() < 100.0 {
                        shopkeeper.set_action(ShopkeeperAction::Walk);
                    }
                }
                ShopkeeperAction::Walk => {
                    transform.translation += difference.normalize() * 1.5;
                }
            }

            shopkeeper.frame += 1;
        }
    }
}
