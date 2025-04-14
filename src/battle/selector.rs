use super::grid::{Grid, GridTranslation};
use crate::actions::{Action, Actions};
use bevy::{math::U8Vec2, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, selector_material)
        .add_systems(Update, (movement, increase_time_since_last_move));
}

#[derive(Resource)]
pub struct SelectorMaterial(pub Handle<StandardMaterial>);

fn selector_material(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(SelectorMaterial(asset_server.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("selector.png")),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    })));
}

/// The players selection.
#[derive(Component, Default)]
#[require(Transform)]
pub struct Selector {
    /// Will pull the tile it is on with it.
    pub pull: bool,
    /// Can the player toggle pull?
    pub pull_locked: bool,

    seconds_since_last_move: f32,
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
    mut selectors: Query<(&mut Selector, &mut GridTranslation)>,
    mut grids: Query<&mut Grid>,
    actions: Actions,
) {
    selectors
        .iter_mut()
        .for_each(|(mut selector, mut translation)| {
            let Ok(grid) = grids.get_mut(translation.grid) else {
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
            let new_translation =
                (translation.translation[direction.0 as usize] as i8 + direction.1) as u8;

            if new_translation >= grid.size[direction.0 as usize] {
                return;
            }

            if selector.pull {
                todo!()
            } else {
                translation.translation[direction.0 as usize] = new_translation;
            }
        });
}
