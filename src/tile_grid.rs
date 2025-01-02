use bevy::prelude::*;
use toolkit::cursor_tracking::CursorPosition;

pub struct TileGridPlugin;

impl Plugin for TileGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridClicked>();
        app.add_systems(Update, grid_clicked);
        app.add_systems(Update, detect_clicked_grid);
    }
}

// Add new system to detect clicked grid
fn detect_clicked_grid(
    cursor_position: Res<CursorPosition>,
    grid_query: Query<(Entity, &GlobalTransform), With<Grid>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut events: EventWriter<GridClicked>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        dbg!(cursor_position.0);
        let clicked = cursor_position.0;

        // Calculate the closest grid tile
        let closest = grid_query
            .iter()
            .map(|(entity, global_transform)| {
                // Calculate de distance from the middle of the grid tile
                let middle = global_transform.translation().xy();
                let distance = middle.distance(clicked);

                (entity, distance)
            })
            .min_by_key(|(_, distance)| (*distance) as i32);

        if let Some((entity, distance)) = closest {
            // Check if it's within threshold
            if distance < 30.0 {
                info!("Clicked grid tile distance: {}", distance);
                events.send(GridClicked(entity));
            }
        }
    }
}

fn grid_clicked(mut commands: Commands, mut event: EventReader<GridClicked>) {
    for evt in event.read() {
        info!("Grid clicked at {:?}", evt.0);

        // Despawn the grid tile
        commands.entity(evt.0).despawn_recursive();
    }
}

#[derive(Event)]
pub struct GridClicked(Entity);

#[derive(Component)]
pub struct Grid;
