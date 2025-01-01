use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct KeyboardMovablePlugin;

#[derive(Component)]
pub struct KeyboardMovable {
    speed: f32,
}

impl KeyboardMovable {
    pub fn new(speed: f32) -> Self {
        KeyboardMovable { speed }
    }
}

impl Default for KeyboardMovable {
    fn default() -> Self {
        KeyboardMovable { speed: 2_500.0 }
    }
}

impl Plugin for KeyboardMovablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement);
        app.add_systems(Update, jump);
    }
}

fn jump(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&mut Velocity, &KeyboardMovable)>,
) {
    if keys.just_pressed(KeyCode::KeyW)
        || keys.just_pressed(KeyCode::ArrowUp)
        || keys.just_pressed(KeyCode::Space)
    {
        if let Ok((mut vel, mov)) = query_player.get_single_mut() {
            vel.linvel.y = 1_000.0;
        }
    }
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&mut Velocity, &KeyboardMovable)>,
) {
    let delta = time.delta_secs();

    let mut direction = Vec2::new(0.0, 0.0);

    if keys.pressed(KeyCode::KeyA) {
        direction += Vec2::new(-1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += Vec2::new(1.0, 0.0);
    }
    if direction.length() == 0.0 {
        return;
    }

    // Normalize
    direction = direction.normalize();

    // Apply changes
    if let Ok((mut vel, mov)) = query_player.get_single_mut() {
        vel.linvel.x = (direction.x * mov.speed);
        println!("Applying velocity {:?}", vel.linvel);
    }
}
