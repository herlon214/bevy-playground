mod keyboard_velocity;

use bevy::audio::Volume;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::RigidBodyBuilder;
use keyboard_velocity::{KeyboardMovable, KeyboardMovablePlugin};
use rand::{self, Rng};

const GRID_SIZE: i32 = 16;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(LevelSelection::index(0))
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(LdtkSettings {
            level_background: LevelBackground::Nonexistent,
            ..default()
        })
        .add_plugins(LdtkPlugin)
        .add_plugins(KeyboardMovablePlugin)
        .init_state::<MyStates>()
        .add_event::<SpawnPlayer>()
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<PlatformBundle>(2)
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<AudioAssets>()
                .load_collection::<SpriteAssets>(),
        )
        .add_systems(Update, (spawn_wall_collision, spawn_player_collision))
        .add_systems(OnEnter(MyStates::Next), start_background_audio)
        .add_systems(Startup, setup)
        // .add_systems(
        //     FixedUpdate,
        //     apply_controls.in_set(TnuaUserControlsSystemSet),
        // )
        .add_systems(Update, camera_follow_player)
        .run();
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
            Collider::capsule(Vec2::new(0.0, -8.0), Vec2::new(0.0, 8.0), 8.0),
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED,
            CameraTarget,
            KeyboardMovable::new(200.0),
            RigidBody::Dynamic,
            GravityScale(3.5),
            Damping {
                linear_damping: 10.0,
                angular_damping: 10.0,
            },
            Friction::new(0.0),
            Ccd::enabled(),
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
            Collider::cuboid(8.0, 8.0),
            RigidBody::Fixed,
            Friction::new(0.0),
            Ccd::enabled(),
        ));
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Platform;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PlatformBundle {
    platform: Platform,
}

#[derive(Component)]
struct CameraTarget;
