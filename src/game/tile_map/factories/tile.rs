use amethyst::{
    prelude::*,
    core::transform::Transform,
    renderer::SpriteRender,
};

use crate::game::tile_map::components::TileTransformable;


pub fn initialise_tile(world: &mut World, x: f32, y: f32, sprite: SpriteRender) {
    let tile_transform = TileTransformable::new(x, y);

    let mut transform = Transform::default();
    transform.set_xyz(tile_transform.get_x_px(), tile_transform.get_y_px(), 0.);

    world
        .create_entity()
        .with(tile_transform)
        .with(transform)
        .with(sprite.clone())
        .build();
}
