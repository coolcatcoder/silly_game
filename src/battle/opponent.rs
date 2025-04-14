use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, move_grids);
}

/// A grid belonging to the opponent.
#[derive(Component, Default)]
pub struct OpponentGrid {
    /// Has the opponent grid collided yet.
    collided: bool,
}

/// When an opponent grid collides with the player grid.
/// Singleton.
#[derive(Component)]
pub struct CollisionEvent {
    opponent: Entity,
}

/// Controls the movement speed of all OpponentGrids.
/// Singleton.
#[derive(Component)]
pub struct OpponentGridSpeed(pub f32);

fn move_grids(
    mut grid: Query<(&mut Transform, &mut OpponentGrid, Entity)>,
    speed: Option<Single<&OpponentGridSpeed>>,
    collision_event: Option<Single<Entity, With<CollisionEvent>>>,
    time: Res<Time>,
    mut commands: ParallelCommands,
    mut c1: Commands,
    mut c2: Commands,
) {
    let Some(speed) = speed.map(|speed| speed.0) else {
        return;
    };

    let collision_event: Option<Entity> = collision_event.map(|collision_event| *collision_event);

    let time_delta_seconds = time.delta_secs();

    grid.par_iter_mut()
        .for_each(|(mut transform, mut grid, entity)| {
            if transform.translation.z <= -30. {
                //info!("Done!");
                return;
            }

            transform.translation.z -= time_delta_seconds * speed;

            if transform.translation.z <= 0. && !grid.collided {
                grid.collided = true;
                //info!("Collided!");

                let to_send = CollisionEvent { opponent: entity };

                // TODO: Send out event.
                match collision_event {
                    Some(collision_event) => {
                        commands.command_scope(|mut commands| {
                            if let Some(mut collision_event) = commands.get_entity(collision_event)
                            {
                                collision_event.insert(to_send);
                            }
                        });
                    }
                    None => {
                        commands.command_scope(|mut commands| {
                            commands.spawn(to_send);
                        });
                    }
                }
            }
        });
}
