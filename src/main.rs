mod player;

use player::{Player, player_movement_system};

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_movement_system.system())
        .add_system(die.system())
        .add_system(take_damage.system())
        .run();
}

struct Skeleton;
struct Health(u64);


fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(50.0, 50.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 50.0)),
            ..Default::default()
        })
        .insert(Skeleton)
        .insert(Health(100));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.9, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(40.0, 40.0)),
            ..Default::default()
        })
        .insert(Player::new())
        .insert(Health(200));
}

fn take_damage(mut entities: Query<&mut Health, With<Skeleton>>) {
    for mut health in entities.iter_mut() {
        health.0 = health.0.saturating_sub(1);
    }
}

fn die(mut commands: Commands, entities: Query<(Entity, &Health)>) {
    for (entity, Health(health)) in entities.iter() {
        if *health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
