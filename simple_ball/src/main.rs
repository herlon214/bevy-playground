use bevy::prelude::*;

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, player_movement)
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
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&Player, &mut Transform)>,
) {
    let speed = 1_000.0;
    let delta = time.delta_secs();

    for (_, mut transform) in &mut query_player {
        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= speed * delta;
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += speed * delta;
        }
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.y += speed * delta;
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation.y -= speed * delta;
        }
    }
}

fn exit_keyboard(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        println!("Exit!");
        exit.send(AppExit::Success);
    }
}
