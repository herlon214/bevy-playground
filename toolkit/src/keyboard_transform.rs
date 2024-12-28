use bevy::prelude::*;

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
    mut query_player: Query<&mut Transform, With<KeyboardMovable>>,
) {
    let speed = 1_000.0;
    let delta = time.delta_secs();

    let mut direction = Vec3::new(0.0, 0.0, 0.0);

    if keys.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyS) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    // Normalize
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    // Apply changes
    for mut transform in &mut query_player {
        transform.translation += direction * speed * delta;
    }
}
