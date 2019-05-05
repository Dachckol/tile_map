use amethyst::{core::transform::Transform, prelude::*, renderer::SpriteRender};

use super::super::components::TileTransformable;

pub fn initialise_tile(world: &mut World, x: u8, y: u8, sprite: SpriteRender) {
    let tile_transform = TileTransformable::new(x, y);
    let transform = Transform::default();

    world
        .create_entity()
        .with(tile_transform)
        .with(transform)
        .with(sprite.clone())
        .build();
}
