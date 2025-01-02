use crate::camera::CameraTarget;
use crate::character::Character;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct VelocityControllerPlugin;

#[derive(Component)]
pub struct VelocityCharacterController {
    speed: f32,
}

impl VelocityCharacterController {
    pub fn new(speed: f32) -> Self {
        VelocityCharacterController { speed }
    }
}

impl Default for VelocityCharacterController {
    fn default() -> Self {
        VelocityCharacterController { speed: 2_500.0 }
    }
}

impl Plugin for VelocityControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, player_movement);
        app.add_systems(FixedUpdate, jump);
        app.add_systems(Update, display_events);
        app.add_systems(Update, on_add_character);
    }
}

fn jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<&mut Velocity, With<VelocityCharacterController>>,
) {
    if keys.just_pressed(KeyCode::KeyW)
        || keys.just_pressed(KeyCode::ArrowUp)
        || keys.just_pressed(KeyCode::Space)
    {
        if let Ok(mut vel) = query_player.get_single_mut() {
            vel.linvel.y = 1_000.0;
        }
    }
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&mut Velocity, &VelocityCharacterController)>,
) {
    let mut direction = Vec2::new(0.0, 0.0);

    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        direction += Vec2::new(-1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        direction += Vec2::new(1.0, 0.0);
    }
    if direction.length() == 0.0 {
        return;
    }

    // Apply changes
    if let Ok((mut vel, mov)) = query_player.get_single_mut() {
        vel.linvel.x = direction.x * mov.speed;
        // println!("Applying velocity {:?}", vel.linvel);
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    // for collision_event in collision_events.read() {
    //     println!("Received collision event: {:?}", collision_event);
    //     match collision_event {
    //         CollisionEvent::Started(lhs, rhs, flags) => {
    //             println!("Collision started: {:?} {:?} {:?}", lhs, rhs, flags);
    //         }
    //         CollisionEvent::Stopped(lhs, rhs, flags) => {
    //             println!("Collision stopped: {:?} {:?} {:?}", lhs, rhs, flags);
    //         }
    //     }
    // }

    // for contact_force_event in contact_force_events.read() {
    //     println!("Received contact force event: {:?}", contact_force_event);
    // }
}

fn on_add_character(mut commands: Commands, query: Query<Entity, Added<Character>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).insert((
            Collider::capsule(Vec2::new(0.0, -8.0), Vec2::new(0.0, 0.0), 8.0),
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED,
            CameraTarget,
            VelocityCharacterController::new(200.0),
            RigidBody::Dynamic,
            // GravityScale(3.5),
            Damping {
                linear_damping: 10.0,
                angular_damping: 10.0,
            },
            Friction::new(0.0),
            Ccd::enabled(),
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}
