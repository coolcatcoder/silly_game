use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, move_grids)
        .add_event::<CollisionEvent>();
}

/// A grid belonging to the opponent.
#[derive(Component, Default)]
pub struct OpponentGrid {
    /// Has the opponent grid collided yet.
    collided: bool,
}

/// When an opponent grid collides with the player grid.
#[derive(Event)]
pub struct CollisionEvent {
    pub opponent: Entity,
}

/// Controls the movement speed of all OpponentGrids.
/// Singleton.
#[derive(Component)]
pub struct OpponentGridSpeed(pub f32);

fn move_grids(
    mut grid: Query<(&mut Transform, &mut OpponentGrid, Entity)>,
    speed: Option<Single<&OpponentGridSpeed>>,
    mut collision_event: EventWriter<CollisionEvent>,
    time: Res<Time>,
) {
    let Some(speed) = speed.map(|speed| speed.0) else {
        return;
    };

    let time_delta_seconds = time.delta_secs();

    grid.iter_mut()
        .for_each(|(mut transform, mut grid, entity)| {
            if transform.translation.z <= -30. {
                //info!("Done!");
                return;
            }

            transform.translation.z -= time_delta_seconds * speed;

            if transform.translation.z <= 0. && !grid.collided {
                grid.collided = true;
                //info!("Collided!");

                collision_event.send(CollisionEvent { opponent: entity });
            }
        });
}
