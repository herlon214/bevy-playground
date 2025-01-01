use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const GRAVITY: f32 = 98.1;
const JUMP_FORCE: f32 = GRAVITY * 20.0;
const MOVE_SPEED: f32 = GRAVITY * 5.0;

#[derive(Component)]
pub struct Grounded;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
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
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut KinematicCharacterController, With<Grounded>>,
) {
    if keys.just_pressed(KeyCode::KeyW)
        || keys.just_pressed(KeyCode::ArrowUp)
        || keys.just_pressed(KeyCode::Space)
    {
        for mut controller in controllers.iter_mut() {
            let jump_vector = Vec2::new(0.0, JUMP_FORCE * time.delta_secs());
            controller.translation = match controller.translation {
                Some(translation) => Some(translation + jump_vector),
                None => Some(jump_vector),
            };
            println!("Jumped");
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
            direction.x *= 0.4;
        }

        // Changed to preserve any existing translation (like jump force)
        controller.translation = match controller.translation {
            Some(translation) => Some(direction + translation),
            None => Some(direction),
        };
    }
}
