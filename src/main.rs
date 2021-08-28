mod collision;
mod player;
mod skeleton;
mod shopkeeper;
mod mouse;

use collision::{debug_hitboxes, debug_hurtboxes, take_damage, physics_system, die_system, HitBoxEvent, Hurtbox};
use player::{player_system, Player, PlayerBundle};
use skeleton::skeleton_system;
use shopkeeper::{shopkeeper_system, ShopkeeperBundle};
use mouse::{MouseState, mouse_system};

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

pub const PLAYER_SPRITE_ROWS: u32 = 2; // TODO: Surely these are redundant and can be derived from the image width / 50...?
pub const PLAYER_SPRITE_COLS: u32 = 2;
pub const MOVEMENT_SPEED :f32 = 6.;
pub const DASH_SPEED :f32 = 50.; // when dashing, vel *= dash_speed
pub const DASH_DURATION :u32 = 6; // dash frame count
pub const DASH_COOLDOWN_TIME : u32 = 60; //frames u need to wait betw dashes
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
                .with_system(player_system.system().label("actions").after("input"))
                .with_system(skeleton_system.system().label("actions"))
                .with_system(shopkeeper_system.system().label("actions"))
                .with_system(die_system.system().label("actions"))
                .with_system(physics_system.system().label("physics").after("actions"))
                .with_system(take_damage.system().after("physics"))
                .with_system(debug_hurtboxes.system().after("physics"))
                .with_system(debug_hitboxes.system().after("physics"))
                .with_system(update_hud.system().after("physics"))
                .with_system(game_over.system())
                .with_system(victory.system()),
        )
        //.insert_resource(ClearColor(Color::rgb(0.6941, 0.2431, 0.3254)))
        .insert_resource(ClearColor(Color::rgb(0.2196, 0.7176, 0.3921)))
        .insert_resource(MouseState::default())
        .run();
}

pub struct MainCamera;

fn setup(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());

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
    }).insert(HUD);

    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "A harmless box...",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::rgb(0.0, 0.0, 0.0),
            },
            Default::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Percent(10.0),
                left: Val::Percent(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(TextBox);

    spawn_entities(commands, materials, asset_server, texture_atlases);
}

fn spawn_entities(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    commands.spawn_bundle(ShopkeeperBundle::new(&mut materials, Vec2::new(400.0, 0.0)));

    let player_texture = asset_server.load("graphics/player.png");
    let player_atlas = TextureAtlas::from_grid(player_texture, Vec2::new(50.0, 50.0), PLAYER_SPRITE_COLS as usize, PLAYER_SPRITE_ROWS as usize);
    let player_atlas = texture_atlases.add(player_atlas);
    commands.spawn_bundle(PlayerBundle::new(player_atlas));
}

struct HUD;
struct TextBox;

fn game_over(
    player: Query<&Player>,
    mut text: Query<&mut Text, With<TextBox>>,
    keyboard_input: Res<Input<KeyCode>>,
    entities: Query<Entity, With<Hurtbox>>,

    // setup
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    if player.single().is_err() {
        if let Ok(mut text) = text.single_mut() {
            text.sections[0].value = format!("Game Over!\nPress 'n' to retry.");
            if keyboard_input.just_pressed(KeyCode::N) {
                for entity in entities.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                spawn_entities(commands, materials, asset_server, texture_atlases);
            }
        }
    }
    else if keyboard_input.get_pressed().count() > 0 {
        if let Ok(mut text) = text.single_mut() {
            text.sections[0].value = format!("");
        }
    }
}

fn victory(
    player: Query<&Hurtbox, Without<Player>>,
    mut text: Query<&mut Text, With<TextBox>>
) {
    if player.iter().count() == 0 {
        if let Ok(mut text) = text.single_mut() {
            text.sections[0].value = format!("A winner is you!");
        }
    }
}

fn update_hud(
    player: Query<(&Hurtbox, &Player)>,
    mut text: Query<&mut Text, With<HUD>>
) {
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
