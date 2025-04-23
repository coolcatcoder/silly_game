use std::any::TypeId;

use super::{
    CubeMesh,
    grid::{Grid, on_grid::OnGrid},
};
use crate::actions::{Action, Actions};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, selector_material)
        .add_systems(Update, (movement, increase_time_since_last_move));
}

#[derive(Resource)]
pub struct SelectorMaterial(pub Handle<StandardMaterial>);

fn selector_material(world: &mut World) {
    let asset_server = world.resource::<AssetServer>();
    let material = SelectorMaterial(asset_server.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("selector.png")),
        unlit: true,
        // TODO: Work out layering, so this is always on top.
        alpha_mode: AlphaMode::Blend,
        ..default()
    }));
    world.insert_resource(material);
}

#[derive(Component, Default)]
pub struct Pullable;

/// There can't ever be 2 of these components with the same TypeId in the same cell.
#[derive(Component, PartialEq, Eq)]
pub struct OnlyOneInCell(pub TypeId);

/// The players selection.
#[derive(Component, Default)]
pub struct Selector {
    /// Will pull the tile it is on with it.
    pub pull: bool,
    /// Can the player toggle pull?
    pub pull_locked: bool,

    seconds_since_last_move: f32,
}

impl Selector {
    pub fn on_grid() -> impl Fn(&mut Commands) -> Entity {
        |commands| {
            let mut selector = commands.spawn(Selector::default());
            selector.queue(|mut selector: EntityWorldMut| {
                let cube_mesh = selector.world().resource::<CubeMesh>().0.clone();
                let selector_material = selector.world().resource::<SelectorMaterial>().0.clone();
                selector.insert((Mesh3d(cube_mesh), MeshMaterial3d(selector_material)));
            });
            selector.id()
        }
    }
}

/// Increases the time since last move.
fn increase_time_since_last_move(mut selectors: Query<&mut Selector>, time: Res<Time>) {
    let time_delta_seconds = time.delta_secs();
    selectors.iter_mut().for_each(|mut selector| {
        selector.seconds_since_last_move += time_delta_seconds;
    });
}

/// Move the selector.
fn movement(
    mut selectors: Query<(Entity, &mut Selector)>,
    pullable: Query<(Option<&OnlyOneInCell>, Has<Pullable>)>,
    mut on_grid: Query<&mut OnGrid>,
    mut grids: Query<&mut Grid>,
    actions: Actions,
) {
    selectors
        .iter_mut()
        .for_each(|(selector_entity, mut selector)| {
            if actions.just_pressed(&Action::Pull) {
                selector.pull = !selector.pull;
            }

            let Ok(mut selector_on_grid) = on_grid.get_mut(selector_entity) else {
                error!("Selector entity is not on grid.");
                return;
            };
            let Ok(mut grid) = grids.get_mut(selector_on_grid.entity()) else {
                error_once!("A selector's grid could not be found.");
                return;
            };

            let mut direction: Option<(u8, i8)> = None;

            // If multiple keys are pressed, we do nothing.
            for (action, action_direction) in [
                (&Action::Up, (1, 1)),
                (&Action::Down, (1, -1)),
                (&Action::Left, (0, 1)),
                (&Action::Right, (0, -1)),
            ] {
                if actions.just_pressed(action)
                    || (selector.seconds_since_last_move > 0.15 && actions.pressed(action))
                {
                    selector.seconds_since_last_move = 0.;
                    match direction {
                        None => {
                            direction = Some(action_direction);
                        }
                        Some(_) => {
                            direction = None;
                            break;
                        }
                    }
                }
            }

            let Some(direction) = direction else {
                return;
            };
            let mut translation = selector_on_grid.translation();
            let new_translation = (translation[direction.0 as usize] as i8 + direction.1) as u8;

            if new_translation >= grid.size()[direction.0 as usize] {
                return;
            }

            translation[direction.0 as usize] = new_translation;

            // if selector.pull {
            //     info!("Start pulling");

            //     let Some(cell) = grid.cells.get(on_grid.index()) else {
            //         error!("Index outside of grid.");
            //         return;
            //     };
            //     // I dislike allocating heap memory every time we move, but I don't see an alternative.
            //     let cell = cell.clone();

            //     cell.iter().copied().for_each(|on_grid| {
            //         let Ok((entity, mut on_grid)) = pullable.get_mut(on_grid) else {
            //             return;
            //         };

            //         on_grid.set_translation(&mut grid, entity, translation);
            //         info!("Pulled.");
            //     });

            //     let Some(grid_cell) = grid.cells.get_mut(on_grid.index()) else {
            //         error!("Index outside of grid.");
            //         return;
            //     };
            // }
            // if selector.pull {
            //     info!("Start pulling");

            //     let index_original = on_grid.index();
            //     on_grid.set_translation(&mut grid, entity, translation);
            //     let index = on_grid.index();

            //     let index_max = index_original.max(index);
            //     let index_min = index_original.min(index);

            //     // Get both cells mutably and safely.
            //     let cells = grid.cells.split_at_mut(index_max);
            //     let Some(cell_1) = cells.0.get_mut(index_min) else {
            //         error!("Index outside of grid.");
            //         return;
            //     };
            //     let Some(cell_2) = cells.1.get_mut(0) else {
            //         error!("Index outside of grid.");
            //         return;
            //     };

            //     // Not sound. Invalidates indices in OnGrid.
            //     std::mem::swap(cell_1, cell_2);
            // } else {
            //     on_grid.set_translation(&mut grid, entity, translation);
            // }

            if selector.pull {
                let index_original = selector_on_grid.index();
                info!("index_original {index_original}");
                let Some(index) = grid.translation_to_index(translation) else {
                    error!("New translation is not in grid.");
                    return;
                };

                let Some(cell_original) = grid.cells().get(index_original) else {
                    error!("Original index is not in grid.");
                    return;
                };
                let Some(cell) = grid.cells().get(index) else {
                    error!("New index is not in grid.");
                    return;
                };

                let mut pull = vec![selector_entity];

                for in_cell_original_entity in cell_original {
                    let Ok(in_cell_original) = pullable.get(*in_cell_original_entity) else {
                        continue;
                    };
                    if in_cell_original.1 {
                        pull.push(*in_cell_original_entity);
                    }

                    for in_cell_entity in cell {
                        let Ok(in_cell) = pullable.get(*in_cell_entity) else {
                            continue;
                        };

                        // If 2 OnlyOneInCell would collide, we give up moving.
                        if let Some(only_1) = in_cell_original.0
                            && let Some(only_2) = in_cell.0
                        {
                            if only_1 == only_2 {
                                return;
                            }
                        }
                    }
                }
                info!("Start pull.");
                pull.into_iter().enumerate().for_each(|(index, entity)| {
                    info!("{index}");
                    let Ok(mut on_grid) = on_grid.get_mut(entity) else {
                        error!("Entity is not in the pullable query.");
                        return;
                    };
                    on_grid.set_translation(&mut grid, entity, translation);
                });
            } else {
                selector_on_grid.set_translation(&mut grid, selector_entity, translation);
            }
        });
}
