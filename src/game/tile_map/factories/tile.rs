use amethyst::{
    prelude::*,
    core::transform::Transform,
    renderer::SpriteRender,
};

use crate::game::tile_map::components::TileTransformable;


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
