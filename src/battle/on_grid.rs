use bevy::{math::U8Vec2, prelude::*};

use super::grid::Grid;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, transforms);
}

#[derive(Component)]
#[require(Transform)]
pub struct OnGrid {
    grid: Entity,
    /// u16 can fit all indices, due to width and height being u8s.
    index: u16,
    /// Redundant information, but it probably saves some performance.
    translation: U8Vec2,
}

impl OnGrid {
    pub fn new(grid: Entity, index: usize, translation: U8Vec2) -> Self {
        Self {
            grid,
            index: index as u16,
            translation,
        }
    }

    pub fn entity(&self) -> Entity {
        self.grid
    }

    pub fn translation(&self) -> U8Vec2 {
        self.translation
    }

    pub fn set_translation(&mut self, grid: &mut Grid, entity: Entity, translation: U8Vec2) {
        let Some(current_cell) = grid.cells.get_mut(self.index as usize) else {
            error!("Index outside grid bounds.");
            return;
        };
        let Some(in_cell_index) = current_cell.iter().position(|e| *e == entity) else {
            error!("Entity not in cell.");
            return;
        };
        current_cell.swap_remove(in_cell_index);

        self.translation = translation;
        let Some(new_index) = grid.translation_to_index(translation) else {
            error!("Translation is outside the grid.");
            return;
        };
        self.index = new_index as u16;

        let Some(new_cell) = grid.cells.get_mut(new_index) else {
            error!("New index outside grid bounds.");
            return;
        };

        new_cell.push(entity);
    }
}

/// Sets transforms to be correct based on their grid.
fn transforms(
    mut transforms: Query<(Ref<OnGrid>, &mut Transform)>,
    grids: Query<(&Transform, &Grid), Without<OnGrid>>,
) {
    transforms
        .par_iter_mut()
        .for_each(|(on_grid, mut transform)| {
            if !on_grid.is_changed() {
                return;
            }
            // info!("Updating grid translation!");

            let Ok((grid_transform, grid)) = grids.get(on_grid.grid) else {
                return;
            };

            // Set each transform to be in the correct 2d translation, and then rotate it around the grid's translation with the grid's rotation.
            transform.translation = grid_transform.translation
                - (grid.size.as_vec2() * 0.5).extend(0.)
                + Vec3::new(
                    on_grid.translation.x as f32 + 0.5,
                    on_grid.translation.y as f32 + 0.5,
                    0.,
                );
            transform.rotate_around(grid_transform.translation, grid_transform.rotation);
        });
}