mod collision;
mod mouse;
mod player;
mod shopkeeper;
mod skeleton;

use bevy::window::WindowMode;
use collision::{
    debug_hitboxes, debug_hurtboxes, die_system, physics_system, take_damage, HitBoxEvent, Hurtbox,
};
use mouse::{mouse_system, MouseState};
use player::{player_system, Player, PlayerBundle};
use shopkeeper::{shopkeeper_system, ShopkeeperBundle};
use skeleton::skeleton_system;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

const WINDOW_WIDTH: f32 = 1024.;
const WINDOW_HEIGHT: f32 = 720.;

const TILE_SCALE: f32 = 8.0;
const TILE_WIDTH: f32 = 8.0;

pub const PLAYER_SPRITE_ROWS: u32 = 2; // TODO: Surely these are redundant and can be derived from the image width / 50...?
pub const PLAYER_SPRITE_COLS: u32 = 2;
pub const MOVEMENT_SPEED: f32 = 6.;
pub const DASH_SPEED: f32 = 50.; // when dashing, vel *= dash_speed
pub const DASH_DURATION: u32 = 6; // dash frame count
pub const DASH_COOLDOWN_TIME: u32 = 60; //frames u need to wait betw dashes
pub const DAMAGE_RECOIL_SPEED: f32 = 15.; // knockback force when u get damaged
pub const DAMAGED_INVINCIBILITY_FRAMES: u32 = 5; // frames that u are invincible after being hit

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Townskeep".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
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
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .insert_resource(MouseState::default())
        .run();
}

pub struct MainCamera;

fn spawn_tiles(
    sprite_indices: Vec<i32>,
    column_number: u32,
    text_atlas_handle: Handle<TextureAtlas>,
    commands: &mut Commands,
) {
    let mut row = 0;
    let mut column = 0;
    let start_pos = Vec2::new(-WINDOW_WIDTH / 2.0 + (TILE_SCALE * TILE_WIDTH) / 2., -WINDOW_HEIGHT / 2.0 + (TILE_SCALE * TILE_WIDTH) / 2.0);

    let mut spsh_bundle = SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: text_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(TILE_SCALE)),
        ..Default::default()
    };

    for ix in sprite_indices {

        if ix >= 0 {
            spsh_bundle.sprite = TextureAtlasSprite::new(ix as u32);
            spsh_bundle.transform.translation = Vec3::new(
                start_pos.x + TILE_SCALE * TILE_WIDTH * row as f32,
                start_pos.y + TILE_SCALE * TILE_WIDTH * column as f32,
                0.0,
            );
            commands.spawn_bundle(spsh_bundle.clone());
        }

        row += 1;

        if row == column_number {
            row = 0;
            column += 1;
        }
    }
}

fn setup(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //Tilemap

    let tilemap_texture = asset_server.load("graphics/Tilemap/colored_tilemap_packed.png");
    let tilemap_atlas =
        TextureAtlas::from_grid(tilemap_texture, Vec2::new(TILE_WIDTH, TILE_WIDTH), 14, 10);
    let tilemap_atlas_handle = texture_atlases.add(tilemap_atlas);

    spawn_tiles(
        vec![
        58, 59, 58, 58, 58, 59, 58, 58, 59, 58, 58, 58, 59, 58, 58, 58,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, 59, -1, -1, 58, -1, -1, 59, -1, -1, -1, -1,
        -1, -1, 58, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, 59, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, 58, -1, -1, -1, -1, -1, 59, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, 58, -1, -1, -1, -1, -1, -1, -1, 58, -1, -1, -1,
        -1, 59, -1, -1, -1, -1, -1, 1, 1, 1, 1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 59, -1,
        -1, -1, -1, -1, -1, -1, -1, 58, -1, -1, -1, -1, -1, -1, -1, -1,
        58, 59, 58, 58, 58, 59, 58, 58, 59, 58, 58, 58, 59, 58, 58, 58,
        58, -1, -1, 58, -1, 59, -1, 58, 59, -1, 58, 58, -1, -1, 58, 58,
        ],
        16,
        tilemap_atlas_handle,
        &mut commands,
    );

    // let mut tilemap = Tilemap::builder()
    //     .auto_spawn(2, 2)
    //     .topology(GridTopology::Square)
    //     .texture_atlas(tilemap_atlas_handle)
    //     .finish()
    //     .unwrap();

    // let tile = Tile {
    //     // 2D location x,y (units are in tiles)
    //     point: (16, 16),

    //     // Which tile from the TextureAtlas
    //     sprite_index: 0,

    //     // Which z-layer in the Tilemap (0-up)
    //     sprite_order: 0,

    //     // Give the tile an optional green tint
    //     tint: bevy::render::color::Color::WHITE,
    // };

    // tilemap.insert_tile(tile).unwrap();

    // let tilemap_components = TilemapBundle {
    //     tilemap,
    //     visible: Visible {
    //         is_visible: true,
    //         is_transparent: true,
    //     },
    //     transform: Default::default(),
    //     global_transform: Default::default(),
    // };

    // commands
    //     .spawn()
    //     .insert_bundle(tilemap_components);

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
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
