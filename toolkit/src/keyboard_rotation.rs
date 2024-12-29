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
    mut query: Query<(&mut Transform, &Rotatable)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::KeyR) {
        for (mut transform, Rotatable(speed)) in query.iter_mut() {
            println!("Rotating platform {}", transform.rotation);
            // Use radians and rotate around Z axis since we're in 2D
            transform.rotate_z(*speed);
        }
    }
}
