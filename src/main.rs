mod camera;
mod character;
mod level;

use bevy::audio::Volume;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use character::controller_kinematic::KinematicControllerPlugin;
use character::controller_velocity::{VelocityCharacterController, VelocityControllerPlugin};
use character::Character;
use level::LevelPlugin;
use rand::{self, Rng};

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
        .add_plugins(LevelPlugin)
        .add_plugins(KinematicControllerPlugin)
        .add_plugins(VelocityControllerPlugin)
        .add_plugins(CameraPlugin)
        .init_state::<MyStates>()
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
        .add_systems(Update, spawn_wall_collision)
        .add_systems(OnEnter(MyStates::Next), start_background_audio)
        .add_systems(Startup, setup)
        // .add_systems(
        //     FixedUpdate,
        //     apply_controls.in_set(TnuaUserControlsSystemSet),
        // )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("gridvania.ldtk").into(),
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
    #[asset(path = "tileset/nature-platformer-tileset-16x16.png")]
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

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Character,
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
