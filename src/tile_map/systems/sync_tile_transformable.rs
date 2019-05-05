use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, System, WriteStorage},
};

use super::super::components::TileTransformable;

pub struct SyncTileTransformable;

impl<'s> System<'s> for SyncTileTransformable {
    type SystemData = (
        WriteStorage<'s, TileTransformable>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut tile_transformables, mut transforms): Self::SystemData) {
        for (tile_transformable, transform) in (&mut tile_transformables, &mut transforms).join() {
            if !tile_transformable.moved {
                continue;
            }
            transform.set_x(tile_transformable.get_x_px());
            transform.set_y(tile_transformable.get_y_px());
            tile_transformable.moved = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amethyst::{ecs::prelude::Entity, prelude::Builder};
    use amethyst_test::{AmethystApplication, EffectReturn};

    #[test]
    fn syncronises_positions_for_moved_tiles_correctly() {
        assert!(AmethystApplication::blank()
            .with_system(SyncTileTransformable, "sync_tile_system", &[])
            .with_effect(|world| {
                let mut transform = Transform::default();
                transform.set_xyz(0., 0., 0.);
                let entity = world
                    .create_entity()
                    .with(transform)
                    .with(TileTransformable::new(4, 7))
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

                assert_eq!(
                    tile_transform.get_x_px(),
                    transform.translation().x,
                    "x values not synced"
                );
                assert_eq!(
                    tile_transform.get_y_px(),
                    transform.translation().y,
                    "y values not synced"
                );
                assert!(!tile_transform.moved);
            })
            .run()
            .is_ok());
    }

    #[test]
    fn ignored_unmoved_tiles() {
        assert!(AmethystApplication::blank()
            .with_system(SyncTileTransformable, "sync_tile_system", &[])
            .with_effect(|world| {
                let mut transform = Transform::default();
                transform.set_xyz(0., 0., 0.);
                let mut transformable = TileTransformable::new(4, 7);
                transformable.moved = false;

                let entity = world
                    .create_entity()
                    .with(transform)
                    .with(transformable)
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

                assert_ne!(tile_transform.get_x_px(), transform.translation().x);
                assert_ne!(tile_transform.get_y_px(), transform.translation().y);
            })
            .run()
            .is_ok());
    }
}
