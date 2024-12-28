use bevy::prelude::*;
use toolkit::keyboard_transform::{KeyboardMovable, KeyboardMovablePlugin};

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(KeyboardMovablePlugin)
        .add_systems(Update, exit_keyboard)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ball)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(Color::hsl(120.0, 1.0, 0.5));
    let mesh = meshes.add(Circle::new(50.0));

    commands.spawn((
        Player,
        KeyboardMovable,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn exit_keyboard(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        println!("Exit!");
        exit.send(AppExit::Success);
    }
}
