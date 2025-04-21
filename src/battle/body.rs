use super::{grid::Grid, opponent::CollisionEvent};
use crate::actions::{Action, Actions};
use bevy::{math::U8Vec2, prelude::*};
use bevy_registration::register;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, material);
    //.add_systems(Update, collide);
}

#[derive(Resource)]
struct Material(Handle<StandardMaterial>);

fn material(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(Material(asset_server.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("selector.png")),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    })));
}

// It should be the other way round. Bodies should never need to collide on their own.
// #[register]
// fn collide(mut collision_event: EventReader<CollisionEvent>, grids: Query<&Grid>) {
//     let collision_event = collision_event.read().next()?;
//     let opponent = grids.get(collision_event.opponent).ok()?;
// }
