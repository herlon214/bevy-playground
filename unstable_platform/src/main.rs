mod keyboard_rotation;
mod keyboard_velocity;

use avian2d::{debug_render::PhysicsDebugPlugin, math::*, prelude::*};
use bevy::audio::Volume;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use keyboard_rotation::{KeyboardRotationPlugin, Rotatable};
use keyboard_velocity::{KeyboardMovable, KeyboardMovablePlugin};
use rand::{self, Rng};
use toolkit::cursor_tracking::{CursorPosition, CursorTrackingPlugin};
use toolkit::fps_counter::FpsCounterPlugin;

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
        .add_plugins(KeyboardMovablePlugin)
        .add_plugins(FpsCounterPlugin)
        .add_plugins(KeyboardRotationPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(CursorTrackingPlugin)
        .init_state::<MyStates>()
        .add_event::<SpawnPlayer>()
        .insert_resource(Gravity(Vector::NEG_Y * 9.81 * 100.0))
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<AudioAssets>()
                .load_collection::<SpriteAssets>(),
        )
        .add_systems(OnEnter(MyStates::Next), start_background_audio)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(MyStates::Next), spawn_ground)
        .add_systems(Update, despawn_out_of_screen)
        // .add_systems(Update, spawn_on_click.run_if(in_state(MyStates::Next)))
        .add_systems(Update, spawn_player)
        .add_systems(OnEnter(MyStates::Next), on_loaded)
        .add_systems(Update, keyboard_inputs)
        .add_systems(Update, player_spawn.run_if(on_event::<SpawnPlayer>))
        .add_systems(Update, on_collision.run_if(on_event::<CollisionStarted>))
        // .add_systems(Update, spawn_on_click.run_if(in_state(MyStates::Next)))
        .run();
}

fn despawn_out_of_screen(
    mut commands: Commands,
    query: Query<(Entity, &Transform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let threshold = 100.0;
    let width = window.width() / 2.0;
    let height = window.height() / 2.0;

    for (entity, transform) in query.iter() {
        if transform.translation.x > width + threshold
            || transform.translation.x < -width - threshold
            || transform.translation.y > height + threshold
            || transform.translation.y < -height - threshold
        {
            commands.entity(entity).despawn_recursive();
            info!(
                "Despawned entity {} at {:?}",
                entity.index(),
                transform.translation
            );
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
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
    for evt in event.read() {
        info!("Collision event: {:?}", evt);
    }
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
                Collider::rectangle(32.0, 32.0), // Example dimensions in meters
                LinearVelocity::ZERO,
                KeyboardMovable::new(2_000.0),
                Rotatable(30.0),
                Visibility::default(),
            ))
            .with_children(|parent| {
                parent.spawn(Sprite::from_image(sprite_assets.bone.clone()));
                parent.spawn(Sprite::from_image(sprite_assets.something.clone()));
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
