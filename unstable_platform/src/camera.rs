use bevy::prelude::*;

#[derive(Component)]
pub struct CameraTarget;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, camera_follow_player);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.4,
            far: 1000.0,
            near: -1000.0,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
    ));
}

fn camera_follow_player(
    time: Res<Time>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<CameraTarget>)>,
    target: Query<&Transform, With<CameraTarget>>,
) {
    let mut camera_transform = camera.single_mut();

    if let Ok(player_transform) = target.get_single() {
        // Define a smaller deadzone where camera won't respond to tiny movements
        let deadzone = 10.0;

        // Calculate the target position (centered on player)
        let target_pos = player_transform.translation;

        // Calculate distance from camera to target
        let distance = target_pos - camera_transform.translation;

        // Only move if we're outside the deadzone
        if distance.length() > deadzone {
            // Smooth camera movement using delta time
            let lerp_speed = 5.0;
            let lerp_factor = (1.0 - (-lerp_speed * time.delta_secs()).exp()).min(1.0);

            camera_transform.translation =
                camera_transform.translation.lerp(target_pos, lerp_factor);
        }
    }
}
