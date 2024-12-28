use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use toolkit::keyboard_velocity::{KeyboardMovable, KeyboardMovablePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(KeyboardMovablePlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        // .add_systems(Update, print_ball_altitude)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d);
}

fn setup_physics(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows.single();
    let width = window.width();
    let height = window.height();

    let thickness = 10.0; // Thickness of the edge boxes

    // Bottom edge
    commands.spawn((
        Collider::cuboid(width / 2.0, thickness / 2.0),
        Transform::from_translation(Vec3::new(0.0, -height / 2.0, 0.0)),
        GlobalTransform::default(),
    ));

    // Top edge
    commands.spawn((
        Collider::cuboid(width / 2.0, thickness / 2.0),
        Transform::from_translation(Vec3::new(0.0, height / 2.0, 0.0)),
        GlobalTransform::default(),
    ));

    // Left edge
    commands.spawn((
        Collider::cuboid(thickness / 2.0, height / 2.0),
        Transform::from_translation(Vec3::new(-width / 2.0, 0.0, 0.0)),
        GlobalTransform::default(),
    ));

    // Right edge
    commands.spawn((
        Collider::cuboid(thickness / 2.0, height / 2.0),
        Transform::from_translation(Vec3::new(width / 2.0, 0.0, 0.0)),
        GlobalTransform::default(),
    ));

    /* Create the ground. */
    commands.spawn((
        Collider::cuboid(500.0, 50.0),
        Transform::from_xyz(0.0, -100.0, 0.0),
    ));

    /* Create the bouncing ball. */
    commands.spawn((
        KeyboardMovable::default(),
        RigidBody::Dynamic,
        Collider::ball(50.0),
        Restitution::coefficient(0.7),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Velocity::zero(),
    ));
}

fn print_ball_altitude(positions: Query<(&Transform, &Velocity), With<RigidBody>>) {
    for (transform, vel) in positions.iter() {
        println!(
            "Ball altitude: {} -- Speed: {}",
            transform.translation, vel.linvel
        );
    }
}
