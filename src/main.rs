use bevy::prelude::*;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Resource)]
struct BallTimer(Timer);

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Animal;

#[derive(Component)]
struct LastBallPosition(f32, f32, f32);

fn main() {
    App::new()
        .insert_resource(BallTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboard_input)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_ball)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(LastBallPosition(0.0, 0.0, 0.0));
}

fn add_people(mut commands: Commands) {
    commands.spawn((Animal, Name("Fritz Kola".to_owned())));
    commands.spawn((Animal, Name("Rabito".to_owned())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Animal>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}", name.0);
        }
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<BallTimer>,
    mut query: Query<&mut LastBallPosition>,
) {
    // Wait for timer
    let delta = time.delta();
    if timer.0.tick(delta).just_finished() {
        // Update position
        let mut lbp = query.single_mut();
        lbp.1 += f32::sin(time.elapsed_secs()) * 10.0;
        lbp.0 += f32::sin(time.elapsed_secs()) * 10.0;

        let material = materials.add(Color::hsl(lbp.1 % 360.0, 1.0, 0.5));
        let mesh = meshes.add(Circle::new(lbp.1));

        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_xyz(lbp.0, lbp.1, lbp.2),
        ));

        println!("Ball position {} {} {}", lbp.0, lbp.1, lbp.2);
        println!("Color {}", lbp.1 / 300.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Animal>>) {
    for mut name in &mut query {
        if name.0 == "Fritz Kola" {
            name.0 = "Fritz Kola The Dog".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        println!("Exit!");
        exit.send(AppExit::Success);
    }
}
