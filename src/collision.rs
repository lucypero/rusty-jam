use bevy::prelude::*;

use bevy::sprite::collide_aabb;
use bevy::math::f32::Vec2;
use bevy_prototype_debug_lines::*;

use crate::{Hurtbox, Skeleton};

pub struct HitBoxEvent {
    pub position: Vec2,
    pub size: Vec2,
    // TODO(rukai): we can add fields to define what kind of damage, knockback etc. should occur
}

// TODO(rukai): this should be made to process hitboxes from any entity type to any entity type. (not just skeletons)
pub fn take_damage(
    mut entities: Query<(&mut Hurtbox, &mut Transform), With<Skeleton>>,
    mut hitbox_events: EventReader<HitBoxEvent>,
) {
    for hitbox in hitbox_events.iter() {
        for (mut hittable, transform) in entities.iter_mut() {
            if collide_aabb::collide(transform.translation, hittable.size, hitbox.position.extend(0.0), hitbox.size).is_some() {
                hittable.health = hittable.health.saturating_sub(1);
            }
        }
    }
}

// TODO(rukai): only include these systems in debug mode
pub fn debug_hurtboxes(
    entities: Query<(&Hurtbox, &Transform), With<Skeleton>>,
    mut lines: ResMut<DebugLines>
) {
    for (hittable, transform) in entities.iter() {
        let pos = transform.translation;
        lines.line(pos, pos + hittable.size.extend(0.0), 0.0);
    }
}

pub fn debug_hitboxes(
    mut hitbox_events: EventReader<HitBoxEvent>,
    mut lines: ResMut<DebugLines>
) {
    for hitbox in hitbox_events.iter() {
        let pos = hitbox.position.extend(0.0);
        lines.line(pos, pos + hitbox.size.extend(0.0), 0.0);
    }
}
