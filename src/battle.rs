use bevy::{math::U8Vec2, prelude::*};
use body::Body;
use grid::Grid;
use opponent::{OpponentGrid, OpponentGridSpeed};
use selector::Selector;

use crate::create_grid;

mod body;
mod grid;
mod on_grid;
mod opponent;
mod selector;

pub fn plugin(app: &mut App) {
    grid::plugin(app);
    on_grid::plugin(app);
    selector::plugin(app);
    opponent::plugin(app);
    body::plugin(app);
    app.add_systems(PreStartup, cube_mesh)
        .add_systems(Startup, experiment);
}

/// Temp battle experiment starter.
fn experiment(mut commands: Commands) {
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
    let s = Selector::on_grid();
    let b = Body::on_grid();
    create_grid!(
        commands
        [s| | | | ]
        [ |b|b| | ]
        [ |b|b|b| ]
        [ | |b|b| ]
        [ | | | | ]
    );

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

fn cube_mesh(world: &mut World) {
    let asset_server = world.resource::<AssetServer>();
    let mesh = CubeMesh(asset_server.add(Cuboid::default().into()));
    world.insert_resource(mesh);
}
