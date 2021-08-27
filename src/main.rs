mod collision;
mod player;
mod mouse;

use collision::{debug_hitboxes, debug_hurtboxes, take_damage, player_take_damage, HitBoxEvent};
use player::{player_movement_system, Player};
use mouse::{MouseState, mouse_system};

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;


pub const ENEMY_SPEED: f32 = 1.0;
pub const MOVEMENT_SPEED :f32 = 3.;
pub const DASH_SPEED :f32 = 30.; // when dashing, vel *= dash_speed
pub const DASH_DURATION :u32 = 6; // dash frame count
pub const DASH_COOLDOWN_TIME : u32 = 60; //frames u need to wait betw dashes
pub const ENEMY_NORMAL_DAMAGE: u64 = 10; //damage that normal enemy attacks deal
pub const DAMAGE_RECOIL_SPEED: f32 = 15.; // knockback force when u get damaged
pub const DAMAGED_INVINCIBILITY_FRAMES : u32 = 5; // frames that u are invincible after being hit

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_event::<HitBoxEvent>()
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(60.0))
                .with_system(mouse_system.system().label("input"))
                .with_system(player_movement_system.system().label("actions").after("input"))
                .with_system(enemy_movement_system.system().label("actions"))
                .with_system(die.system().label("actions"))
                .with_system(take_damage.system().after("actions"))
                .with_system(player_take_damage.system().after("actions"))
                .with_system(debug_hurtboxes.system().after("actions"))
                .with_system(debug_hitboxes.system().after("actions"))
                .with_system(update_hud.system().after("actions")),
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(MouseState::default())
        .run();
}

pub struct MainCamera;

pub struct Skeleton;

pub struct Hurtbox {
    pub size: Vec2,
    health: u64,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(MainCamera);
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

    let player_texture = asset_server.load("graphics/player.png");
    let player_atlas = TextureAtlas::from_grid(player_texture, Vec2::new(50.0, 50.0), 2, 2);
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform::from_scale(Vec3::splat(3.0)),
            texture_atlas: texture_atlases.add(player_atlas),
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
    if let Ok((Hurtbox { health, .. }, player)) = player.single() {
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

fn die(
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

fn enemy_movement_system(
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
