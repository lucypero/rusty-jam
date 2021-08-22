mod player;
mod collision;

use collision::{HitBoxEvent, take_damage, debug_hurtboxes, debug_hitboxes};
use player::{Player, player_movement_system};

use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy_prototype_debug_lines::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_event::<HitBoxEvent>()
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(60.0))
                .with_system(player_movement_system.system().label("actions"))
                .with_system(die.system().label("actions"))
                .with_system(take_damage.system().after("actions"))
                .with_system(debug_hurtboxes.system().after("actions"))
                .with_system(debug_hitboxes.system().after("actions"))
                .with_system(update_hud.system().after("actions"))
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .run();
}

pub struct Skeleton;
pub struct Hurtbox {
    pub size: Vec2,
    health: u64
}

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
        .insert(Hurtbox {
            size: Vec2::new(30.0, 50.0),
            health: 1,
        });
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(50.0, 50.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 50.0)),
            ..Default::default()
        })
        .insert(Skeleton)
        .insert(Hurtbox {
            size: Vec2::new(30.0, 50.0),
            health: 1,
        });

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.9, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(40.0, 40.0)),
            ..Default::default()
        })
        .insert(Player::new())
        .insert(Hurtbox {
            size: Vec2::new(30.0, 50.0),
            health: 200,
        });

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

fn update_hud(player: Query<(&Hurtbox, &Player)>, mut text: Query<&mut Text>) {
    if let Ok((Hurtbox {health, ..}, player)) = player.single() {
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

fn die(mut commands: Commands, entities: Query<(Entity, &Hurtbox)>, mut player: Query<&mut Player>) {
    for (entity, Hurtbox { health , ..}) in entities.iter() {
        if *health <= 0 {
            commands.entity(entity).despawn_recursive();
            if let Ok(mut player) = player.single_mut() {
                player.exp += 100;
                player.money += 200;
            }
        }
    }
}
