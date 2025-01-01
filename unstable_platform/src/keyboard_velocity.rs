use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
        app.add_systems(FixedUpdate, player_movement);
        app.add_systems(FixedUpdate, jump);
        app.add_systems(Update, display_events);
    }
}

fn jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<&mut Velocity, With<KeyboardMovable>>,
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
    mut query_player: Query<(&mut Velocity, &KeyboardMovable)>,
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
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(lhs, rhs, flags) => {
                println!("Collision started: {:?} {:?} {:?}", lhs, rhs, flags);
            }
            CollisionEvent::Stopped(lhs, rhs, flags) => {
                println!("Collision stopped: {:?} {:?} {:?}", lhs, rhs, flags);
            }
        }
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
