use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::prelude::*;

use crate::character::Character;

const ASPECT_RATIO: f32 = 16. / 9.0;

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
            scale: 0.3,
            far: 1000.0,
            near: -1000.0,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(0.0, 16.0 * 8.0, 0.0),
    ));
}

fn camera_follow_player(
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &OrthographicProjection), (Without<CameraTarget>)>,
    target: Query<&Transform, (With<CameraTarget>, Without<Camera2d>)>,
    level_query: Query<
        (&Transform, &LevelIid),
        (Without<OrthographicProjection>, Without<Character>),
    >,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_selection: Res<LevelSelection>,
) {
    let (mut camera_transform, projection) = camera.single_mut();

    // Get current level boundaries
    let mut level_min_x = f32::NEG_INFINITY;
    let mut level_max_x = f32::INFINITY;
    let mut px_wid = 16.0;

    // Extract level boundaries from LDtk project
    for (level_transform, level_iid) in &level_query {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        if level_selection.is_match(&LevelIndices::default(), level) {
            level_min_x = level_transform.translation.x;
            level_max_x = level_transform.translation.x + (level.px_wid as f32);
        }
    }

    if let Ok(player_transform) = target.get_single() {
        let deadzone = 10.0;

        // Adjust the level boundaries to account for camera view
        let camera_min_x = level_min_x + (px_wid * 12.0);
        let camera_max_x = level_max_x - (px_wid * 12.0);

        // Center the player by removing the x-offset and maintaining y position
        let target_pos = Vec3::new(
            player_transform.translation.x,
            camera_transform.translation.y,
            camera_transform.translation.z,
        );

        let distance = target_pos - camera_transform.translation;

        if distance.length() > deadzone {
            let lerp_speed = 5.0;
            let lerp_factor = (1.0 - (-lerp_speed * time.delta_secs()).exp()).min(1.0);

            // First lerp to the target
            let new_pos = camera_transform.translation.lerp(target_pos, lerp_factor);

            // Then clamp the camera position to prevent seeing outside the level
            camera_transform.translation.x = new_pos.x.clamp(camera_min_x, camera_max_x);
            camera_transform.translation.y = new_pos.y;
            camera_transform.translation.z = new_pos.z;

            println!("Final camera position: {}", camera_transform.translation.x);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Character>,
    >,
    player_query: Query<&Transform, With<Character>>,
    level_query: Query<
        (&Transform, &LevelIid),
        (Without<OrthographicProjection>, Without<Character>),
    >,
    ldtk_projects: Query<&LdtkProjectHandle>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_iid) in &level_query {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_projects.single())
                .expect("Project should be loaded if level has spawned");

            let level = ldtk_project
                .get_raw_level_by_iid(&level_iid.to_string())
                .expect("Spawned level should exist in LDtk project");

            if level_selection.is_match(&LevelIndices::default(), level) {
                let level_ratio = level.px_wid as f32 / level.px_hei as f32;
                orthographic_projection.viewport_origin = Vec2::ZERO;
                if level_ratio > ASPECT_RATIO {
                    // level is wider than the screen
                    let height = (level.px_hei as f32 / 9.).round() * 9.;
                    let width = height * ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.x =
                        (player_translation.x - level_transform.translation.x - width / 2.)
                            .clamp(0., level.px_wid as f32 - width);
                    camera_transform.translation.y = 0.;
                } else {
                    // level is taller than the screen
                    let width = (level.px_wid as f32 / 16.).round() * 16.;
                    let height = width / ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.y =
                        (player_translation.y - level_transform.translation.y - height / 2.)
                            .clamp(0., level.px_hei as f32 - height);
                    camera_transform.translation.x = 0.;
                }

                camera_transform.translation.x += level_transform.translation.x;
                camera_transform.translation.y += level_transform.translation.y;
            }
        }
    }
}
