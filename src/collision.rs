use bevy::prelude::*;

use bevy::sprite::collide_aabb;
use bevy::math::f32::Vec2;
use crate::{Hurtbox, Skeleton};

pub struct HitBoxEvent {
    pub position: Vec2,
    pub size: Vec2,
    // TODO: we can add fields to define what kind of damage, knockback etc. should occur
}

// TODO: this should be made to process hitboxes from any entity type to any entity type. (not just skeletons)
// TODO: maybe the easiest way to add debug display for hitboxes is to make them entities themself. Shouldnt be that different from sending a message if I just delete it after its proceessed.
//       Or maybe I just use this instead https://crates.io/crates/bevy_prototype_debug_lines and draw a box outline with it.
//       I think that makes more sense as bevy should handle this use case ... it just doesnt yet.
pub fn take_damage(mut entities: Query<(&mut Hurtbox, &mut Transform), With<Skeleton>>, mut hitbox_events: EventReader<HitBoxEvent>) {
    for hitbox in hitbox_events.iter() {
        for (mut hittable, transform) in entities.iter_mut() {
            if collide_aabb::collide(transform.translation, hittable.size, hitbox.position.extend(0.0), hitbox.size).is_some() {
                hittable.health = hittable.health.saturating_sub(1);
            }
        }
    }
}
