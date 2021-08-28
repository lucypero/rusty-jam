use bevy::math::vec3;
use bevy::prelude::*;

use bevy::math::f32::Vec2;
use bevy::sprite::collide_aabb;
use bevy_prototype_debug_lines::*;

use crate::player::Player;

pub enum Team {
    Player,
    Enemy,
}

pub struct Hurtbox {
    pub team: Team,
    pub size: Vec2,
    pub health: u64,
    pub is_hit: bool,
    pub invincible: bool,
    pub vel: Vec2, // TODO: maybe split it into a Physics component? not sure if its worth it.
}

pub enum CanHitTeam {
    Player,
    Enemy,
    //All,
}
impl CanHitTeam {
    fn can_hit(&self, team: &Team) -> bool {
        match (self, team) {
            (CanHitTeam::Enemy, Team::Enemy) => true,
            (CanHitTeam::Player, Team::Player) => true,
            //(CanHitTeam::All, Team::Player) => true,
            //(CanHitTeam::All, Team::Enemy) => true,
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

pub fn take_damage(
    mut entities: Query<(&mut Hurtbox, &mut Transform)>,
    mut hitbox_events: EventReader<HitBoxEvent>,
) {
    for hitbox in hitbox_events.iter() {
        for (mut hurtbox, transform) in entities.iter_mut() {
            if hitbox.can_hit.can_hit(&hurtbox.team) && !hurtbox.invincible &&
                collide_aabb::collide(
                    transform.translation,
                    hurtbox.size,
                    hitbox.position.extend(0.0),
                    hitbox.size,
                )
                .is_some()
            {
                hurtbox.is_hit = true;
                hurtbox.invincible = true;
                hurtbox.health = hurtbox.health.saturating_sub(hitbox.damage);
                let direction = transform.translation.truncate() - hitbox.position;
                hurtbox.vel = direction.normalize() * hitbox.knockback;
            }
        }
    }
}

pub fn physics_system(mut entities: Query<(&mut Hurtbox, &mut Transform)>) {
    for (mut hurtbox, mut transform) in entities.iter_mut() {
        //apply vel and friction
        transform.translation += hurtbox.vel.extend(0.0);
        hurtbox.vel *= 0.8;
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
    entities: Query<(&Hurtbox, &Transform), Without<Player>>,
    mut lines: ResMut<DebugLines>,
) {
    for (hittable, transform) in entities.iter() {
        let size = hittable.size;
        let pos = transform.translation;

        draw_box(&mut lines, pos, size, Color::YELLOW);
    }
}

pub fn debug_hitboxes(mut hitbox_events: EventReader<HitBoxEvent>, mut lines: ResMut<DebugLines>) {
    for hitbox in hitbox_events.iter() {
        let pos = hitbox.position.extend(0.0);
        let size = hitbox.size;
        draw_box(&mut lines, pos, size, Color::RED);
    }
}

fn draw_box(lines: &mut DebugLines, pos: Vec3, size: Vec2, color: Color) {
    let size = size.extend(0.0);
    let pos = pos - size / 2.0;
    let p1 = pos;
    let p2 = pos + vec3(size.x, 0.0, 0.0);
    let p3 = pos + vec3(0.0, size.y, 0.0);
    let p4 = pos + size;
    lines.line_colored(p1, p2, 0.0, color);
    lines.line_colored(p2, p4, 0.0, color);
    lines.line_colored(p1, p3, 0.0, color);
    lines.line_colored(p3, p4, 0.0, color);
}
