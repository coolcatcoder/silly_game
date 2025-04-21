use super::CubeMesh;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, material);
    //.add_systems(Update, collide);
}

/// A player's body block.
#[derive(Component, Default)]
pub struct Body;

impl Body {
    pub fn on_grid() -> impl Fn(&mut Commands) -> Entity {
        |commands| {
            let mut entity = commands.spawn(Body);
            entity.queue(|mut entity: EntityWorldMut| {
                let cube_mesh = entity.world().resource::<CubeMesh>().0.clone();
                let selector_material = entity.world().resource::<Material>().0.clone();
                entity.insert((Mesh3d(cube_mesh), MeshMaterial3d(selector_material)));
            });
            entity.id()
        }
    }
}

#[derive(Resource)]
struct Material(Handle<StandardMaterial>);

fn material(world: &mut World) {
    let asset_server = world.resource::<AssetServer>();
    let material = Material(asset_server.add(StandardMaterial {
        base_color_texture: None,
        base_color: Color::BLACK,
        unlit: true,
        alpha_mode: AlphaMode::Opaque,
        ..default()
    }));
    world.insert_resource(material);
}

// It should be the other way round. Bodies should never need to collide on their own.
// #[register]
// fn collide(mut collision_event: EventReader<CollisionEvent>, grids: Query<&Grid>) {
//     let collision_event = collision_event.read().next()?;
//     let opponent = grids.get(collision_event.opponent).ok()?;
// }
