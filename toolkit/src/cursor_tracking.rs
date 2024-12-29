use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource, Default, Debug)]
pub struct CursorPosition(pub Vec2);

pub struct CursorTrackingPlugin;

impl Plugin for CursorTrackingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, cursor_system);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(CursorPosition::default());
}

fn cursor_system(
    mut cursor: ResMut<CursorPosition>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    let world_pos = window
        .cursor_position()
        .and_then(|cursor| {
            Some(
                camera
                    .viewport_to_world(camera_transform, cursor)
                    .expect("viewport conversion error")
                    .origin
                    .truncate(),
            )
        })
        .unwrap_or_default();

    cursor.0 = world_pos;
}
