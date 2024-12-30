use avian2d::prelude::*;
use bevy::prelude::*;

pub struct KeyboardRotationPlugin;

#[derive(Component)]
pub struct Rotatable(pub f32);

impl Plugin for KeyboardRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotate_platform);
    }
}

fn rotate_platform(
    mut query: Query<(&mut AngularVelocity, &Rotatable)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if keys.pressed(KeyCode::KeyR) {
        for (mut vel, Rotatable(speed)) in query.iter_mut() {
            vel.0 += *speed * time.delta_secs();
        }
    }
}
