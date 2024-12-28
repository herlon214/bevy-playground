use bevy::prelude::*;
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
        .add_systems(Update, print_ball_altitude)
        .add_systems(Update, reset_ball)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d);
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands.spawn((
        Collider::cuboid(500.0, 50.0),
        Transform::from_xyz(0.0, -100.0, 0.0),
    ));

    /* Create the bouncing ball. */
    commands.spawn((
        KeyboardMovable,
        RigidBody::Dynamic,
        Collider::ball(50.0),
        Restitution::coefficient(0.7),
        Transform::from_xyz(0.0, 400.0, 0.0),
        Velocity::zero(),
    ));
}

fn reset_ball(mut positions: Query<(&mut Transform, &mut Velocity), With<RigidBody>>) {
    for (mut trans, mut vel) in positions.iter_mut() {
        if trans.translation.y < -50.0 {
            trans.translation = Vec3::new(0.0, 0.0, 0.0);
            *vel = Velocity::zero();
        }
    }
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
