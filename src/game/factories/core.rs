use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, Projection},
};

use crate::game::components::Controllable;
use crate::resources::sprites::SpriteStore;
use crate::tile_map::{commands::TileTranslation, components::TileTransformable};

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, 1024., 0.0, 1024.,
        )))
        .with(transform)
        .build();
}

pub fn initialise_player(world: &mut World, sprite_store: &SpriteStore) {
    world
        .create_entity()
        .with(TileTransformable::new(16, 16))
        .with(Transform::default())
        .with(Controllable::new(vec![
            ("move_up", TileTranslation::UP(1)),
            ("move_down", TileTranslation::DOWN(1)),
            ("move_left", TileTranslation::LEFT(1)),
            ("move_right", TileTranslation::RIGHT(1)),
        ]))
        .with(sprite_store.get_player().clone())
        .build();
}
