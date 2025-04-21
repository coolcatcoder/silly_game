use bevy::prelude::*;

pub trait ExtensionCreate {
    fn create<T: Create>(&mut self, with: T::In) -> Entity;
}

impl ExtensionCreate for Commands<'_, '_> {
    fn create<T: Create>(&mut self, with: T::In) -> Entity {
        let entity = self.spawn_empty().id();
        self.queue(move |world: &mut World| T::create(entity, world, with));
        entity
    }
}

pub trait Create {
    type In: Send + Sync + 'static;
    //type Out;
    fn create(entity: Entity, world: &mut World, with: Self::In);
}
