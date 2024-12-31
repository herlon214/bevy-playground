mod keyboard_rotation;
mod keyboard_velocity;
mod tile_grid;

use std::collections::{HashMap, HashSet};

use avian2d::{debug_render::PhysicsDebugPlugin, math::*, prelude::*};
use bevy::audio::Volume;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use keyboard_rotation::{KeyboardRotationPlugin, Rotatable};
use keyboard_velocity::{KeyboardMovable, KeyboardMovablePlugin};
use rand::{self, Rng};
use tile_grid::{Grid, TileGridPlugin};
use toolkit::cursor_tracking::{CursorPosition, CursorTrackingPlugin};
const GRID_SIZE: i32 = 16;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::WHITE),
                ..default()
            },
            GizmoConfig::default(),
        )
        .insert_resource(LevelSelection::index(0))
        // .add_plugins(TileGridPlugin)
        .add_plugins(KeyboardMovablePlugin)
        .add_plugins(KeyboardRotationPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(CursorTrackingPlugin)
        .insert_resource(LdtkSettings {
            level_background: LevelBackground::Nonexistent,
            ..default()
        })
        .add_plugins(LdtkPlugin)
        .init_state::<MyStates>()
        .add_event::<SpawnPlayer>()
        .insert_resource(Gravity(Vector::NEG_Y * 9.81 * 100.0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .register_ldtk_int_cell::<WallBundle>(1)
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<AudioAssets>()
                .load_collection::<SpriteAssets>(),
        )
        .add_systems(Update, spawn_wall_collision)
        .add_systems(OnEnter(MyStates::Next), start_background_audio)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_player_collision)
        // .add_systems(OnEnter(MyStates::Next), spawn_ground)
        // .add_systems(Update, despawn_out_of_screen)
        // .add_systems(Update, spawn_on_click.run_if(in_state(MyStates::Next)))
        // .add_systems(Update, spawn_player)
        // .add_systems(OnEnter(MyStates::Next), on_loaded)
        // .add_systems(Update, keyboard_inputs)
        // .add_systems(Update, player_spawn.run_if(on_event::<SpawnPlayer>))
        // .add_systems(Update, on_collision.run_if(on_event::<CollisionStarted>))
        // .add_systems(Startup, screen_grid)
        // .add_systems(Update, render_rays)
        .add_systems(Update, camera_follow_player)
        // .add_systems(Update, spawn_on_click.run_if(in_state(MyStates::Next)))
        .run();
}

fn screen_grid(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows.single();

    let mut rng = rand::thread_rng();

    // Create grids
    let grid_size = 64;
    let width = (window.width() / 2.0) as i32;
    let height = (window.height() / 2.0) as i32;
    for x in (-width..width).step_by(grid_size) {
        for y in (-height..height).step_by(grid_size) {
            commands.spawn((
                Transform::from_xyz(x as f32, y as f32, 0.0),
                Grid,
                Sprite::from_color(
                    Color::hsla(rng.gen_range(0.0..360.0), 1.0, 0.5, 0.1),
                    Vec2::new(grid_size as f32, grid_size as f32),
                ),
            ));
        }
    }
}

fn camera_follow_player(
    time: Res<Time>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<CameraTarget>)>,
    target: Query<&Transform, With<CameraTarget>>,
) {
    let mut camera_transform = camera.single_mut();

    if let Ok(player_transform) = target.get_single() {
        // Define a smaller deadzone where camera won't respond to tiny movements
        let deadzone = 10.0;

        // Calculate the target position (centered on player)
        let target_pos = player_transform.translation;

        // Calculate distance from camera to target
        let distance = target_pos - camera_transform.translation;

        // Only move if we're outside the deadzone
        if distance.length() > deadzone {
            // Smooth camera movement using delta time
            let lerp_speed = 5.0;
            let lerp_factor = (1.0 - (-lerp_speed * time.delta_secs()).exp()).min(1.0);

            camera_transform.translation =
                camera_transform.translation.lerp(target_pos, lerp_factor);
        }
    }
}

fn despawn_out_of_screen(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Despawnable>>,
    camera: Query<&Transform, With<Camera2d>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let camera_transform = camera.single();
    let threshold = 100.0;
    let width = window.width() / 2.0;
    let height = window.height() / 2.0;

    // Calculate camera-relative bounds
    let camera_pos = camera_transform.translation.truncate();
    let min_x = camera_pos.x - width - threshold;
    let max_x = camera_pos.x + width + threshold;
    let min_y = camera_pos.y - height - threshold;
    let max_y = camera_pos.y + height + threshold;

    for (entity, transform) in query.iter() {
        let pos = transform.translation;
        if pos.x > max_x || pos.x < min_x || pos.y > max_y || pos.y < min_y {
            commands.entity(entity).despawn_recursive();
            info!(
                "Despawned entity {} at {:?}",
                entity.index(),
                transform.translation
            );
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn(Camera2d);
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.4,
            far: 1000.0,
            near: -1000.0,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk").into(),
        ..Default::default()
    });
}

fn on_loaded(mut events: EventWriter<SpawnPlayer>) {
    events.send(SpawnPlayer(Vec2::new(0.0, 0.0)));
}

fn keyboard_inputs(keys: Res<ButtonInput<KeyCode>>, mut spawn_evt: EventWriter<SpawnPlayer>) {
    if keys.pressed(KeyCode::Space) {
        spawn_evt.send(SpawnPlayer(Vec2::new(0.0, 0.0)));
    }
}

fn on_collision(mut event: EventReader<CollisionStarted>) {
    // for evt in event.read() {
    // info!("Collision event: {:?}", evt);
    // }
}

fn player_spawn(mut event: EventReader<SpawnPlayer>) {
    for evt in event.read() {
        info!("Spawned player at {:?}", evt.0);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut event: EventReader<SpawnPlayer>,
    sprite_assets: Res<SpriteAssets>,
) {
    let mut rng = rand::thread_rng();

    for evt in event.read() {
        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        let pos = Vec2::new(x, y) + evt.0;

        // Spawn the platform entity
        commands
            .spawn((
                Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(3.0)),
                RigidBody::Dynamic,
                Collider::capsule(3.0, 32.0),
                LinearVelocity::ZERO,
                KeyboardMovable::new(2_000.0),
                Rotatable(30.0),
                Visibility::default(),
                RayCaster::new(Vector::ZERO, Dir2::X),
                Player,
                Despawnable,
            ))
            .with_children(|parent| {
                parent.spawn(Sprite::from_image(sprite_assets.bone.clone()));
            });

        debug!("Spawned player at {:?}", pos);
    }
}

fn spawn_ground(
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();

    // Calculate the bottom of the screen based on the camera's orthographic projection
    let bottom_y = -(window.height() / 2.0); // Convert to world coordinates

    let width = (window.width() / 2.0) as i32;
    for i in (-width..width).step_by(32) {
        commands.spawn(sprite_bundle(
            &sprite_assets,
            1,
            Vec2::new(i as f32, bottom_y),
        ));

        println!("Spawned sprite at {:?}", Vec2::new(i as f32, 0.0));
    }
}

fn sprite_bundle(
    sprite_assets: &Res<SpriteAssets>,
    index: usize,
    position: Vec2,
) -> (Sprite, RigidBody, Collider, Transform) {
    return (
        Sprite::from_atlas_image(
            sprite_assets.sprite.clone(),
            TextureAtlas {
                layout: sprite_assets.layout.clone(),
                index,
            },
        ),
        RigidBody::Static,
        Collider::rectangle(16.0, 16.0),
        Transform::from_xyz(position.x + 16.0, position.y + 16.0, 0.0).with_scale(Vec3::splat(2.0)),
    );
}

fn spawn_on_click(
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
    cursor_position: Res<CursorPosition>,
    keys: Res<ButtonInput<MouseButton>>,
) {
    if keys.just_pressed(MouseButton::Left) || keys.just_pressed(MouseButton::Right) {
        commands.spawn(sprite_bundle(&sprite_assets, 0, cursor_position.0));
        println!("Cursor sprite at {:?}", cursor_position.0);
    }
}

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "audio/theme.ogg")]
    background: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
struct SpriteAssets {
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 7, rows = 12))]
    layout: Handle<TextureAtlasLayout>,
    #[asset(path = "tileset/nature-paltformer-tileset-16x16.png")]
    sprite: Handle<Image>,

    #[asset(path = "tinyswords/Deco/14.png")]
    bone: Handle<Image>,

    #[asset(path = "tinyswords/Deco/16.png")]
    something: Handle<Image>,
}

/// This system runs in MyStates::Next. Thus, AudioAssets is available as a resource
/// and the contained handle is done loading.
fn start_background_audio(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    let settings = PlaybackSettings::LOOP.with_volume(Volume::new(0.1));

    commands.spawn((AudioPlayer(audio_assets.background.clone()), settings));
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

#[derive(Event)]
struct SpawnPlayer(Vec2);

fn render_rays(mut rays: Query<(&mut RayCaster, &mut RayHits)>, mut gizmos: Gizmos) {
    for (ray, hits) in &mut rays {
        // Convert to Vec3 for lines
        let origin = ray.global_origin().f32();
        let direction = ray.global_direction().f32();

        for hit in hits.iter() {
            gizmos.line_2d(
                origin,
                origin + direction * hit.distance as f32,
                Color::WHITE,
            );
        }
        if hits.is_empty() {
            gizmos.line_2d(origin, origin + direction * 1_000_000.0, Color::BLACK);
        }
    }
}

#[derive(Component, Default)]
struct Player;

#[derive(Component)]
struct Despawnable;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
    }
}

fn spawn_player_collision(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &mut Transform),
        (Added<Player>, With<GridCoords>, Without<RigidBody>),
    >,
) {
    if let Ok((entity, mut transform)) = player_query.get_single_mut() {
        println!("Spawned player collision at {:?}", transform);

        // Update transform Z to be in front of the camera
        transform.translation.z = 1.0;

        commands.entity(entity).insert((
            Collider::rectangle(16.0, 32.0),
            RigidBody::Dynamic,
            LinearVelocity::ZERO,
            KeyboardMovable::new(2_000.0),
            LockedAxes::ROTATION_LOCKED,
            CameraTarget,
        ));
    }
}

pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<&Transform, (Added<Wall>, With<GridCoords>, Without<RigidBody>)>,
) {
    for transform in wall_query.iter() {
        println!("Spawned wall at {:?}", transform);
        commands.spawn((
            Transform::from_xyz(
                transform.translation.x + 8.0, // Pivot point is in the center of the tile
                transform.translation.y + 8.0, // Pivot point is in the center of the tile
                0.0,
            ),
            Collider::rectangle(16.0, 16.0),
            RigidBody::Static,
        ));
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Component)]
struct CameraTarget;
