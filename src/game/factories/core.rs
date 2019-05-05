use amethyst::{
    prelude::*,
    core::transform::Transform,
    renderer::{Camera, Projection},
};

use crate::game::tile_map::components::TileTransformable;
use crate::game::resources::sprites::SpriteStore;

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            1024.,
            0.0,
            1024.,
        )))
        .with(transform)
        .build();
}

pub fn initialise_player(world: &mut World, sprite_store: &SpriteStore) {
    let transform = Transform::default();
    let tile_transform = TileTransformable::new(16,16);

    world
        .create_entity()
        .with(tile_transform)
        .with(transform)
        .with(sprite_store.get_player().clone())
        .build();
}
