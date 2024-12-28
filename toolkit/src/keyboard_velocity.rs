use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

const SPEED: f32 = 2_000.0;

pub struct KeyboardMovablePlugin;

#[derive(Component)]
pub struct KeyboardMovable;

impl Plugin for KeyboardMovablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement);
    }
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<&mut Velocity, With<KeyboardMovable>>,
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
    for mut vel in &mut query_player {
        vel.linvel += direction * SPEED * delta;
    }
}
