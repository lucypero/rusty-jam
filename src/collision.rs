use bevy::math::{Vec3Swizzles, vec3};
use bevy::prelude::*;

use bevy::math::f32::Vec2;
use bevy::sprite::collide_aabb;
use bevy_prototype_debug_lines::*;

use crate::{DAMAGE_RECOIL_SPEED, ENEMY_NORMAL_DAMAGE, player::Player};
use crate::skeleton::Skeleton;

pub enum Team {
    Player,
    Enemy,
}

pub struct Hurtbox {
    pub team: Team,
    pub size: Vec2,
    pub health: u64,
}

pub enum CanHitTeam {
    //Player,
    Enemy,
    //All,
}
impl CanHitTeam {
    fn can_hit(&self, team: &Team) -> bool {
        match (self, team) {
            (CanHitTeam::Enemy, Team::Enemy) => true,
            _ => false,
        }
    }
}

pub struct HitBoxEvent {
    pub position: Vec2,
    pub size: Vec2,
    pub damage: u64,
    pub knockback: f32,
    pub can_hit: CanHitTeam,
}

// TODO(rukai): this should be made to process hitboxes from any entity type to any entity type. (not just skeletons)
pub fn take_damage(
    mut entities: Query<(&mut Hurtbox, &mut Transform), With<Skeleton>>,
    mut hitbox_events: EventReader<HitBoxEvent>,
) {
    for hitbox in hitbox_events.iter() {
        for (mut hurtbox, transform) in entities.iter_mut() {
            if hitbox.can_hit.can_hit(&hurtbox.team) && collide_aabb::collide(
                transform.translation,
                hurtbox.size,
                hitbox.position.extend(0.0),
                hitbox.size,
            )
            .is_some()
            {
                hurtbox.health = hurtbox.health.saturating_sub(hitbox.damage);
            }
        }
    }
}

pub fn player_take_damage(
    mut skeleton_q: Query<(&mut Hurtbox, &mut Transform), With<Skeleton>>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut Hurtbox), Without<Skeleton>>,
) {
    if let Ok((mut player, player_transform, mut player_hb)) = player_query.single_mut() {
        for (skel_hb, skel_trans) in skeleton_q.iter_mut() {
            if !player.invincible && collide_aabb::collide(
                skel_trans.translation,
                skel_hb.size,
                player_transform.translation,
                player_hb.size,
            )
            .is_some()
            {
                let mut vec = player_transform.translation - skel_trans.translation;
                vec = vec.normalize() * DAMAGE_RECOIL_SPEED;

                player.take_damage(vec.xy());
                player_hb.health = player_hb.health.saturating_sub(ENEMY_NORMAL_DAMAGE);
            }
        }
    }
}

pub fn die_system(
    mut commands: Commands,
    entities: Query<(Entity, &Hurtbox)>,
    mut player: Query<&mut Player>,
) {
    for (entity, Hurtbox { health, .. }) in entities.iter() {
        if *health <= 0 {
            commands.entity(entity).despawn_recursive();
            if let Ok(mut player) = player.single_mut() {
                player.exp += 100;
                player.money += 200;
            }
        }
    }
}

// TODO(rukai): only include these systems in debug mode
pub fn debug_hurtboxes(
    entities: Query<(&Hurtbox, &Transform), With<Skeleton>>,
    mut lines: ResMut<DebugLines>,
) {
    for (hittable, transform) in entities.iter() {
        let size = hittable.size;
        let pos = transform.translation;

        draw_box(&mut lines, pos, size);
    }
}

pub fn debug_hitboxes(mut hitbox_events: EventReader<HitBoxEvent>, mut lines: ResMut<DebugLines>) {
    for hitbox in hitbox_events.iter() {
        let pos = hitbox.position.extend(0.0);
        let size = hitbox.size;
        draw_box(&mut lines, pos, size);
    }
}

fn draw_box(lines: &mut DebugLines, pos: Vec3, size: Vec2) {
    let size = size.extend(0.0);
    let pos = pos - size / 2.0;
    let p1 = pos;
    let p2 = pos + vec3(size.x, 0., 0.0);
    let p3 = pos + vec3(0., size.y, 0.0);
    let p4 = pos + size;
    lines.line(p1, p2, 0.);
    lines.line(p2, p4, 0.);
    lines.line(p1, p3, 0.);
    lines.line(p3, p4, 0.);
}
