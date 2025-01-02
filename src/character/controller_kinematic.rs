use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const GRAVITY: f32 = 98.1;
const JUMP_FORCE: f32 = 4_000.0;
const MOVE_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct DoubleJump;

pub struct KinematicControllerPlugin;

impl Plugin for KinematicControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                remove_grounded,
                add_grounded,
                jump,
                player_movement,
                gravity,
                read_kineamtic_controller,
            ),
        );
    }
}

fn add_grounded(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Grounded>>,
) {
    if let Ok((entity, controller)) = query.get_single() {
        if controller.grounded || controller.effective_translation.y.abs() < 0.01 {
            commands.entity(entity).insert(Grounded);
            commands.entity(entity).remove::<DoubleJump>();
            println!("Grounded");
        }
    }
}

fn remove_grounded(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), With<Grounded>>,
) {
    if let Ok((entity, controller)) = query.get_single() {
        if !controller.grounded && controller.effective_translation.y.abs() > 0.01 {
            commands.entity(entity).remove::<Grounded>();
            println!("On Air");
        }
    }
}

fn gravity(time: Res<Time>, mut query: Query<&mut KinematicCharacterController>) {
    for mut controller in query.iter_mut() {
        let gravity_vector = Vec2::new(0.0, -GRAVITY * time.delta_secs());

        controller.translation = match controller.translation {
            Some(translation) => Some(translation + gravity_vector),
            None => Some(gravity_vector),
        };
    }
}

fn read_kineamtic_controller(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    // for (entity, output) in controllers.iter() {
    //     println!(
    //         "Entity {:?} moved by {:?} and touches the ground: {:?}",
    //         entity, output.effective_translation, output.grounded
    //     );
    // }
}

fn jump(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<
        (Entity, &mut KinematicCharacterController, Option<&Grounded>),
        Without<DoubleJump>,
    >,
) {
    let mut direction = Vec2::ZERO;

    if keys.just_pressed(KeyCode::KeyW)
        || keys.just_pressed(KeyCode::ArrowUp)
        || keys.just_pressed(KeyCode::Space)
    {
        direction += Vec2::new(0.0, 1.0);
    }

    if direction.length() == 0.0 {
        return;
    }

    // Check is moving forward or backward
    if keys.pressed(KeyCode::KeyD) {
        direction += Vec2::new(1.0, 0.0);
    }

    if keys.pressed(KeyCode::KeyA) {
        direction += Vec2::new(-1.0, 0.0);
    }

    // Normalize the direction
    direction = direction.normalize() * JUMP_FORCE * time.delta_secs();

    for (entity, mut controller, grounded) in controllers.iter_mut() {
        let jump_vector = direction;
        controller.translation = match controller.translation {
            Some(translation) => Some(translation + jump_vector),
            None => Some(jump_vector),
        };

        if grounded.is_none() {
            commands.entity(entity).insert(DoubleJump);
            println!("Double Jump");
        } else {
            println!("Single Jump");
        }
    }
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<(&mut KinematicCharacterController, Option<&Grounded>)>,
) {
    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyA) {
        direction.x -= MOVE_SPEED;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += MOVE_SPEED;
    }

    if direction.length() == 0.0 {
        return;
    }

    direction = direction * time.delta_secs();

    for (mut controller, grounded) in controllers.iter_mut() {
        // Reduce horizontal speed when in air
        if grounded.is_none() {
            direction.x *= 0.8;
        }

        // Changed to preserve any existing translation (like jump force)
        controller.translation = match controller.translation {
            Some(translation) => Some(direction + translation),
            None => Some(direction),
        };
    }
}
