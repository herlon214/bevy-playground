use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub struct KeyboardRotationPlugin;

#[derive(Component)]
pub struct Rotatable(pub f32);

impl Plugin for KeyboardRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotate_platform);
    }
}

fn rotate_platform(mut query: Query<(&mut Velocity, &Rotatable)>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::KeyR) {
        for (mut vel, Rotatable(speed)) in query.iter_mut() {
            println!("Rotating platform {}", vel.angvel);
            vel.angvel += *speed;
        }
    }
}
