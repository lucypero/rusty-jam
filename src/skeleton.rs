use bevy::prelude::*;
use crate::player::Player;
use crate::Hurtbox;

pub const ENEMY_SPEED: f32 = 1.0;

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
            skeleton: Skeleton,
            hurtbox: Hurtbox {
                size: Vec2::new(30.0, 50.0),
                health: 1,
            },
            sprite: SpriteBundle {
                material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                transform: Transform::from_translation(position.extend(0.0)),
                sprite: Sprite::new(Vec2::new(30.0, 50.0)),
                ..Default::default()
            }
        }
    }
}

pub struct Skeleton;

pub fn skeleton_system(
    mut player_query: Query<(&mut Player, &Transform)>,
    mut enemy_query: Query<(&mut Skeleton, &mut Transform), Without<Player>>,
    // mut q: QuerySet<(
    //     Query<(&Player, &Transform)>,
    //     Query<(&Skeleton, &mut Transform), Without<Player>>
    // )>
) {
    if let Ok((_player, player_transform)) = player_query.single_mut() {
        for (_enemy, mut en_transform) in enemy_query.iter_mut() {
            let mut vec = player_transform.translation - en_transform.translation;
            vec = vec.normalize() * ENEMY_SPEED;
            en_transform.translation += vec;
        }
    }

    // if let Ok((mut player, mut player_transform)) = q.q0().single() {
    //     for mut skeleton_t in q.q1_mut().iter_mut() {

    //     }
    // }
}
