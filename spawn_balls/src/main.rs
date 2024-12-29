use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use toolkit::fps_counter::FpsCounterPlugin;
use toolkit::keyboard_velocity::{KeyboardMovable, KeyboardMovablePlugin};

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
    pub enabled: bool,
    pub ball_size: f32,
}

#[derive(Component)]
pub struct Lifetime(Timer);

fn main() {
    App::new()
        .insert_resource(SpawnTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            enabled: true,
            ball_size: 10.0,
        })
        .add_plugins(FpsCounterPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(KeyboardMovablePlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, spawn_ball)
        .add_systems(Update, despawn_ball)
        .add_systems(Update, toggle_spawn_timer)
        // .add_systems(Update, print_ball_altitude)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d);
}

fn spawn_ball(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    let mut rng = thread_rng();

    if timer.enabled && timer.timer.tick(time.delta()).just_finished() {
        // Randomize
        let vel = Velocity::linear(Vec2::new(
            rng.gen_range(-3_000..3_000) as f32,
            rng.gen_range(-3_000..3_000) as f32,
        ));
        let max_size = timer.ball_size.max(5.0);
        let size = rng.gen_range(max_size..max_size * 2.0) as f32;

        commands.spawn((
            Lifetime(Timer::from_seconds(15.0, TimerMode::Once)),
            RigidBody::Dynamic,
            Collider::ball(size),
            Restitution::coefficient(0.9),
            Transform::from_xyz(0.0, 0.0, 0.0),
            vel,
        ));
    }
}

fn despawn_ball(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lt) in query.iter_mut() {
        if lt.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
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

pub fn toggle_spawn_timer(mut timer: ResMut<SpawnTimer>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        timer.enabled = !timer.enabled;
    }

    if keys.just_pressed(KeyCode::Digit0) {
        timer.ball_size += 10.0;
    }
    if keys.just_pressed(KeyCode::Digit1) {
        timer.ball_size -= 10.0;
    }
}
