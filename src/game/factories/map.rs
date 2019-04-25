use amethyst::{
    prelude::*,
    core::transform::Transform,
    renderer::SpriteRender,
};

use crate::game::resources::sprites::SpriteStore;
use crate::game::components::map::TileTransform;

pub const MAP_WIDTH: f32 = 32.;
pub const MAP_HEIGHT: f32 = 32.;

pub fn initialise_map(world: &mut World, sprite_store: &SpriteStore) {
    world.register::<TileTransform>();
    for y in 0..(MAP_HEIGHT as usize) {
        for x in 0..(MAP_WIDTH as usize) {
            initialise_tile(world, x as f32, y as f32, sprite_store.get_ground());
        }
    }
}

fn initialise_tile(world: &mut World, x: f32, y: f32, sprite: SpriteRender) {
    let tile_transform = TileTransform::new(x, y);

    let mut transform = Transform::default();
    transform.set_xyz(tile_transform.get_x_px(), tile_transform.get_y_px(), 0.);

    world
        .create_entity()
        .with(tile_transform)
        .with(transform)
        .with(sprite.clone())
        .build();
}
