use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const GRAVITY: f32 = 98.1;
const JUMP_FORCE: f32 = 3000.0;
const MOVE_SPEED: f32 = 800.0;
const TERMINAL_VELOCITY: f32 = -1000.0;

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (gravity, jump, player_movement));
    }
}

fn gravity(time: Res<Time>, mut query: Query<&mut KinematicCharacterController>) {
    for mut controller in query.iter_mut() {
        match controller.translation {
            Some(translation) => {
                controller.translation =
                    Some(translation + Vec2::new(0.0, -GRAVITY * time.delta_secs()));
            }
            None => {
                controller.translation = Some(Vec2::new(0.0, -GRAVITY * time.delta_secs()));
            }
        }
    }
}

// fn read_kineamtic_controller(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
//     for (entity, output) in controllers.iter() {
//         println!(
//             "Entity {:?} moved by {:?} and touches the ground: {:?}",
//             entity, output.effective_translation, output.grounded
//         );
//     }
// }

fn jump(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut KinematicCharacterController>,
) {
    if keys.just_pressed(KeyCode::KeyW)
        || keys.just_pressed(KeyCode::ArrowUp)
        || keys.just_pressed(KeyCode::Space)
    {
        for mut controller in controllers.iter_mut() {
            controller.translation = Some(Vec2::new(0.0, JUMP_FORCE * time.delta_secs()));
        }
    }
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut KinematicCharacterController>,
) {
    let mut direction = Vec2::new(0.0, 0.0);

    if keys.pressed(KeyCode::KeyA) {
        direction += Vec2::new(-MOVE_SPEED, 0.0);
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += Vec2::new(MOVE_SPEED, 0.0);
    }
    if keys.pressed(KeyCode::KeyW) {
        direction += Vec2::new(0.0, MOVE_SPEED);
    }
    if keys.pressed(KeyCode::KeyS) {
        direction += Vec2::new(0.0, -MOVE_SPEED);
    }

    if direction.length() == 0.0 {
        return;
    }

    for mut controller in controllers.iter_mut() {
        match controller.translation {
            Some(translation) => {
                controller.translation = Some(translation + (direction * time.delta_secs()));

                println!("Moving player, {:?}", translation);
            }
            None => {
                controller.translation = Some(direction * time.delta_secs());
            }
        }
    }
}
