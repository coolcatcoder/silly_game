use super::{CubeMesh, grid::Grid, on_grid::OnGrid};
use crate::actions::{Action, Actions};
use bevy::{math::U8Vec2, prelude::*};

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
        alpha_mode: AlphaMode::Blend,
        ..default()
    }));
    world.insert_resource(material);
}

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
    mut selectors: Query<(Entity, &mut Selector, &mut OnGrid)>,
    mut grids: Query<&mut Grid>,
    actions: Actions,
) {
    selectors
        .iter_mut()
        .for_each(|(entity, mut selector, mut on_grid)| {
            let Ok(mut grid) = grids.get_mut(on_grid.entity()) else {
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
                    || (selector.seconds_since_last_move > 0.1 && actions.pressed(action))
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
            let mut translation = on_grid.translation();
            let new_translation = (translation[direction.0 as usize] as i8 + direction.1) as u8;

            if new_translation >= grid.size[direction.0 as usize] {
                return;
            }

            if selector.pull {
                todo!()
            } else {
                translation[direction.0 as usize] = new_translation;
                on_grid.set_translation(&mut grid, entity, translation);
                info!("?");
            }
        });
}
