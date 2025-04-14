use crate::battle::FromBattle;
use bevy::{math::U8Vec2, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (cells_transforms, debug, grid_translation));
}

#[derive(Component)]
#[require(Transform, FromBattle)]
pub struct Grid {
    pub cells: Box<[Option<Entity>]>,
    pub size: U8Vec2,
}

impl Grid {
    pub fn new(size: U8Vec2) -> Self {
        Self {
            cells: vec![None; (size.x * size.y).into()].into_boxed_slice(),
            size,
        }
    }

    /// Sets the cell at the translation.
    /// This will error if the translation is invalid.
    pub fn set(mut self, translation: U8Vec2, cell: Option<Entity>) -> Self {
        let Some(index) = self.translation_to_index(translation) else {
            error!("Invalid translation!");
            return self;
        };
        self.cells[index as usize] = cell;
        self
    }

    /// Converts the grid index to the grid translation.
    /// Returns None if the index is outside the grid.
    fn index_to_translation(&self, index: u8) -> Option<U8Vec2> {
        if index >= self.size.x * self.size.y {
            return None;
        }

        let translation = U8Vec2::new(
            // If you imagine every index in 1 line, then if you wrap the index back to 0 every time it reaches width, you will have x.
            index % self.size.x,
            // TODO: Explain how this works.
            index / self.size.x,
        );

        Some(translation)
    }

    /// Convert from a translation in grid space to an index for a the flattened array that represents the grid.
    /// Returns None if the translation is outside the grid.
    fn translation_to_index(&self, translation: U8Vec2) -> Option<u8> {
        if translation.x >= self.size.x || translation.y >= self.size.y {
            return None;
        }

        let index = translation.y * self.size.x + translation.x;
        Some(index)
    }
}

/// Sets cells' transforms to be correct based on their grid.
fn cells_transforms(
    grids: Query<(&Grid, Ref<Transform>)>,
    mut transforms: Query<&mut Transform, Without<Grid>>,
) {
    grids.iter().for_each(|(grid, grid_transform)| {
        if !grid_transform.is_changed() {
            return;
        }
        // info!("Updating grid cell!");
        grid.cells.iter().enumerate().for_each(|(index, cell)| {
            let Some(cell) = cell else {
                return;
            };

            let Ok(mut transform) = transforms.get_mut(*cell) else {
                return;
            };

            // SAFETY: All indices are in the grid.
            let translation = grid.index_to_translation(index as u8).unwrap();

            // Set each cell to be in the correct 2d translation, and then rotate it around the grid's translation with the grid's rotation.
            transform.translation = grid_transform.translation
                + Vec3::new(translation.x as f32, translation.y as f32, 0.);
            transform.rotate_around(grid_transform.translation, grid_transform.rotation);
        });
    });
}

// Debug gizmos for the grid.
fn debug(mut gizmos: Gizmos, grids: Query<(&Grid, &Transform)>) {
    grids.iter().for_each(|(grid, transform)| {
        gizmos
            .grid(
                Isometry3d::new(
                    transform.translation, //+ (grid.size.as_vec2() * 0.5).extend(0.),
                    transform.rotation,
                ),
                grid.size.into(),
                Vec2::splat(1.),
                Color::BLACK,
            )
            .outer_edges();
    });
}

/// TODO: I think this is a mistake.
/// I have both grid cells and grid translations.
/// I should pick one and stick with it.
#[deprecated]
#[derive(Component)]
pub struct GridTranslation {
    pub grid: Entity,
    pub translation: U8Vec2,
}

/// Set the GridTranslations' transforms to be correct.
fn grid_translation(
    mut transforms: Query<(Ref<GridTranslation>, &mut Transform)>,
    grids: Query<(&Transform, &Grid), Without<GridTranslation>>,
) {
    transforms
        .par_iter_mut()
        .for_each(|(translation, mut transform)| {
            if !translation.is_changed() {
                return;
            }
            // info!("Updating grid translation!");

            let Ok((grid_transform, grid)) = grids.get(translation.grid) else {
                return;
            };

            // Set each transform to be in the correct 2d translation, and then rotate it around the grid's translation with the grid's rotation.
            transform.translation = grid_transform.translation
                - (grid.size.as_vec2() * 0.5).extend(0.)
                + Vec3::new(
                    translation.translation.x as f32 + 0.5,
                    translation.translation.y as f32 + 0.5,
                    0.,
                );
            transform.rotate_around(grid_transform.translation, grid_transform.rotation);
        });
}
