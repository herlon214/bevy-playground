use std::borrow::Borrow;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use toolkit::cursor_tracking::{CursorPosition, CursorTrackingPlugin};
use toolkit::keyboard_rotation::{KeyboardRotationPlugin, Rotatable};
use toolkit::keyboard_velocity::{KeyboardMovable, KeyboardMovablePlugin};

#[derive(Component)]
pub struct Platform(Vec2);

#[derive(Resource)]
pub struct NatureAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(KeyboardMovablePlugin)
        .add_plugins(KeyboardRotationPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(CursorTrackingPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, platform_position)
        .add_systems(Startup, spawn_sprites.after(setup))
        .add_systems(Update, spawn_on_click)
        .run();
}

fn platform_position(query: Query<&Transform, With<Platform>>) {
    // for transform in query.iter() {
    //     println!("Platform position: {:?}", transform.translation);
    // }
}

fn setup(
    mut commands: Commands,
    rapier_query: Query<&RapierConfiguration>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let rapier_config = rapier_query
        .get_single()
        .expect("Rapier configuration not found");

    let texture: Handle<Image> = asset_server.load("tileset/nature-paltformer-tileset-16x16.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 7, 11, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let nature_atlas = NatureAtlas {
        texture,
        layout: texture_atlas_layout,
    };

    commands.insert_resource(nature_atlas);

    // Spawn the platform entity
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(100.0, 10.0), // Example dimensions in meters
        Transform::from_xyz(0.0, 5.0, 0.0), // Initial position in meters
        Platform(Vec2::new(0.0, 5.0)),
        Velocity::zero(),
        KeyboardMovable::new(2_000.0),
        Rotatable(0.1),
        Ccd::enabled(),
    ));
}

fn spawn_sprites(
    mut commands: Commands,
    nature_atlas: Res<NatureAtlas>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();

    // Calculate the bottom of the screen based on the camera's orthographic projection
    let bottom_y = -(window.height() / 2.0); // Convert to world coordinates

    let width = (window.width() / 2.0) as i32;
    for i in (-width..width).step_by(32) {
        commands.spawn(sprite_bundle(
            nature_atlas.borrow(),
            0,
            Vec2::new(i as f32, bottom_y),
        ));

        println!("Spawned sprite at {:?}", Vec2::new(i as f32, 0.0));
    }
}

fn sprite_bundle(
    nature_atlas: &Res<NatureAtlas>,
    index: usize,
    position: Vec2,
) -> (Sprite, RigidBody, Collider, Transform) {
    return (
        Sprite::from_atlas_image(
            nature_atlas.texture.clone(),
            TextureAtlas {
                layout: nature_atlas.layout.clone(),
                index,
            },
        ),
        RigidBody::Fixed,
        Collider::cuboid(8.0, 8.0),
        Transform::from_xyz(position.x + 16.0, position.y + 16.0, 0.0).with_scale(Vec3::splat(2.0)),
    );
}

fn spawn_on_click(
    mut commands: Commands,
    nature_atlas: Res<NatureAtlas>,
    cursor_position: Res<CursorPosition>,
    keys: Res<ButtonInput<MouseButton>>,
) {
    if keys.just_pressed(MouseButton::Left) || keys.just_pressed(MouseButton::Right) {
        commands.spawn(sprite_bundle(nature_atlas.borrow(), 0, cursor_position.0));
        println!("Cursor sprite at {:?}", cursor_position.0);
    }
}
