use crate::battle::FromBattle;
use bevy::{math::U8Vec2, prelude::*};

pub mod on_grid;

pub fn plugin(app: &mut App) {
    on_grid::plugin(app);
    app.add_systems(Update, debug);
}

#[derive(Component)]
#[require(Transform, FromBattle)]
pub struct Grid {
    cells: Box<[Vec<Entity>]>,
    size: U8Vec2,
}

impl Grid {
    pub fn new(size: U8Vec2) -> Self {
        Self {
            cells: vec![Vec::new(); (size.x * size.y).into()].into_boxed_slice(),
            size,
        }
    }

    /// Creates a new grid with the cells set.
    /// Will error if cells' length does not match size.
    pub fn new_with_cells(cells: Box<[Vec<Entity>]>, size: U8Vec2) -> Self {
        if cells.len() as u16 != size.x as u16 * size.y as u16 {
            error!("Size does not match length!");
        }
        Self { cells, size }
    }

    pub fn size(&self) -> U8Vec2 {
        self.size
    }

    pub fn cells(&self) -> &[Vec<Entity>] {
        &self.cells
    }

    /// Converts the grid index to the grid translation.
    /// Returns None if the index is outside the grid.
    pub fn index_to_translation(&self, index: usize) -> Option<U8Vec2> {
        let index = index as u16;
        if index >= self.size.x as u16 * self.size.y as u16 {
            return None;
        }

        let translation = U8Vec2::new(
            // If you imagine every index in 1 line, then if you wrap the index back to 0 every time it reaches width, you will have x.
            (index % self.size.x as u16) as u8,
            // TODO: Explain how this works.
            (index / self.size.x as u16) as u8,
        );

        Some(translation)
    }

    /// Convert from a translation in grid space to an index for a the flattened array that represents the grid.
    /// Returns None if the translation is outside the grid.
    pub fn translation_to_index(&self, translation: U8Vec2) -> Option<usize> {
        if translation.x >= self.size.x || translation.y >= self.size.y {
            return None;
        }

        let index = translation.y * self.size.x + translation.x;
        Some(index as usize)
    }
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

pub trait ToGridCell {
    fn to_grid_cell(self) -> Vec<Entity>;
}
impl ToGridCell for &mut Commands<'_, '_> {
    fn to_grid_cell(self) -> Vec<Entity> {
        Vec::new()
    }
}
impl ToGridCell for Vec<Entity> {
    fn to_grid_cell(self) -> Vec<Entity> {
        self
    }
}
impl ToGridCell for Entity {
    fn to_grid_cell(self) -> Vec<Entity> {
        vec![self]
    }
}

#[macro_export]
macro_rules! create_grid {
    (
        $commands:ident
        $([
            $(
                $($contained:ident)?
            )|*
        ])*
    ) => {
        {
            let rows = [$(
                [$(
                    crate::battle::grid::ToGridCell::to_grid_cell($($contained)?(&mut $commands))
                ),*]
            ),*];

            let size = U8Vec2::new(rows[0].len() as u8, rows.len() as u8);
            let grid = Grid::new_with_cells(rows.into_iter().flatten().collect(), size);
            let grid_entity = $commands.spawn_empty().id();

            grid.cells().iter().enumerate().for_each(|(index, cell)| {
                let translation = grid.index_to_translation(index).unwrap();

                cell.iter().for_each(|entity| {
                    $commands.entity(*entity).insert(crate::battle::grid::on_grid::OnGrid::new(grid_entity, index, translation));
                });
            });

            $commands.entity(grid_entity).insert(grid);
        }
    };
}
