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
        .add_system(update_hud.system())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .run();
}

struct Skeleton;
struct Health(u64);

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(-50.0, -400.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 50.0)),
            ..Default::default()
        })
        .insert(Skeleton)
        .insert(Health(700));
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(50.0, 50.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 50.0)),
            ..Default::default()
        })
        .insert(Skeleton)
        .insert(Health(500));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.9, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(40.0, 40.0)),
            ..Default::default()
        })
        .insert(Player::new())
        .insert(Health(200));

    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Temp HUD:",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(1.0, 0.61, 0.0),
            },
            Default::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn take_damage(mut entities: Query<&mut Health, With<Skeleton>>) {
    for mut health in entities.iter_mut() {
        health.0 = health.0.saturating_sub(1);
    }
}

fn update_hud(player: Query<(&Health, &Player)>, mut text: Query<&mut Text>) {
    if let Ok((Health(health), player)) = player.single() {
        if let Ok(mut text) = text.single_mut() {
            text.sections[0].value = format!(
                "Health: {}\nMoney: {}\nEXP: {}/{}\nLevel: {}",
                health,
                player.money,
                player.exp,
                100,
                player.level()
            );
        }
    }
}

fn die(mut commands: Commands, entities: Query<(Entity, &Health)>, mut player: Query<&mut Player>) {
    for (entity, Health(health)) in entities.iter() {
        if *health <= 0 {
            commands.entity(entity).despawn_recursive();
            if let Ok(mut player) = player.single_mut() {
                player.exp += 100;
                player.money += 200;
            }
        }
    }
}
