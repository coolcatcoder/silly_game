use bevy::{math::U8Vec2, prelude::*};
use grid::{Grid, GridTranslation};
use opponent::{OpponentGrid, OpponentGridSpeed};
use selector::{Selector, SelectorMaterial};

mod grid;
mod opponent;
mod selector;

pub fn plugin(app: &mut App) {
    grid::plugin(app);
    selector::plugin(app);
    opponent::plugin(app);
    app.add_systems(Startup, cube_mesh)
        .add_systems(Update, experiment);
}

/// Temp battle experiment starter.
fn experiment(
    mut commands: Commands,
    cube_mesh: Option<Res<CubeMesh>>,
    selector_material: Option<Res<SelectorMaterial>>,
    mut finished: Local<bool>,
) {
    if *finished {
        return;
    }

    let Some(cube_mesh) = cube_mesh else {
        return;
    };
    let Some(selector_material) = selector_material else {
        return;
    };
    *finished = true;

    commands.spawn((
        Transform {
            translation: Vec3::new(-10., 0., -10.),
            rotation: Quat::from_euler(EulerRot::XYZ, 0., (-90_f32 + -60.).to_radians(), 0.),
            ..default()
        },
        Camera3d { ..default() },
        // Perhaps not.
        // Projection::Orthographic(OrthographicProjection {
        //     scale: 0.01,
        //     ..OrthographicProjection::default_3d()
        // }),
    ));

    // player
    let player_grid = commands.spawn(Grid::new(U8Vec2::splat(5))).id();
    commands.spawn((
        Selector::default(),
        GridTranslation {
            grid: player_grid,
            translation: U8Vec2::ZERO,
        },
        Mesh3d(cube_mesh.0.clone()),
        MeshMaterial3d(selector_material.0.clone()),
    ));

    // opponent
    commands.spawn((
        Grid::new(U8Vec2::splat(5)),
        Transform::from_xyz(0., 0., 20.),
        OpponentGrid::default(),
    ));

    commands.spawn(OpponentGridSpeed(2.5));
}

/// All entities related to the battle will have this.
/// They can then be cleaned up easily.
#[derive(Component, Default)]
struct FromBattle;

#[derive(Resource)]
struct CubeMesh(Handle<Mesh>);

fn cube_mesh(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(CubeMesh(asset_server.add(Cuboid::default().into())));
}
