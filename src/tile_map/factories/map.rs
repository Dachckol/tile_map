use amethyst::{
    prelude::*,
    core::transform::Transform,
};

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;

use crate::tile_map::components::tile::{Tile};

pub fn initialise_map(world: &mut World) {
    world.register::<Tile>();
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            initialise_tile(world, x, y);
        }
    }
}

fn initialise_tile(world: &mut World, x: usize, y: usize) {
    let tile = Tile::new(x, y);

    let mut transform = Transform::default();
    transform.set_xyz(tile.get_x_px() as f32, tile.get_y_px() as f32, 0.);

    world
        .create_entity()
        .with(tile)
        .with(transform)
        .build();
}
