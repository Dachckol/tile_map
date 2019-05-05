use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::game::tile_map::components::TileTransformable;

pub struct SyncTileTransformable;

impl<'s> System<'s> for SyncTileTransformable {
    type SystemData = (
        ReadStorage<'s, TileTransformable>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (tile_transforms, mut transforms): Self::SystemData) {
        for (tile, transform) in (&tile_transforms, &mut transforms).join() {
            transform.set_x(tile.get_x_px());
            transform.set_y(tile.get_y_px());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SyncTileTransformable;
    use super::TileTransformable;
    use super::Transform;
    use amethyst::{ecs::prelude::Entity, prelude::Builder};
    use amethyst_test::{AmethystApplication, EffectReturn};

    #[test]
    fn syncronises_positions_correctly() {
        assert!(AmethystApplication::blank()
            .with_system(SyncTileTransformable, "sync_tile_system", &[])
            .with_effect(|world| {
                let mut transform = Transform::default();
                transform.set_xyz(0., 0., 0.);
                let entity = world
                    .create_entity()
                    .with(transform)
                    .with(TileTransformable::new(4., 7.))
                    .build();
                world.add_resource(EffectReturn(entity))
            })
            .with_assertion(|world| {
                let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();

                let transform_store = world.read_storage::<Transform>();
                let tile_transformable_store = world.read_storage::<TileTransformable>();

                let transform = transform_store
                    .get(entity)
                    .expect("Entity should have a `Transform` component.");
                let tile_transform = tile_transformable_store
                    .get(entity)
                    .expect("Entity should have a `TileTransformable` component.");

                assert_eq!(tile_transform.get_x_px(), transform.translation().x);
                assert_eq!(tile_transform.get_y_px(), transform.translation().y);
            })
            .run()
            .is_ok()
        );
    }
}
