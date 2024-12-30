use avian2d::prelude::*;
use bevy::prelude::*;

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
    }
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&mut LinearVelocity, &KeyboardMovable)>,
) {
    let delta = time.delta_secs();

    let mut direction = Vec2::new(0.0, 0.0);

    if keys.pressed(KeyCode::KeyA) {
        direction += Vec2::new(-1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += Vec2::new(1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyW) {
        direction += Vec2::new(0.0, 1.0);
    }
    if keys.pressed(KeyCode::KeyS) {
        direction += Vec2::new(0.0, -1.0);
    }

    // Normalize
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    // Apply changes
    for (mut vel, mov) in &mut query_player {
        vel.0 += direction * mov.speed * delta;
    }
}
