use amethyst::{
    prelude::*,
};

use crate::game::resources::sprites::SpriteStore;
use crate::game::tile_map::components::TileTransformable;
use super::tile::initialise_tile;

pub const MAP_WIDTH: f32 = 32.;
pub const MAP_HEIGHT: f32 = 32.;

pub fn initialise_map(world: &mut World, sprite_store: &SpriteStore) {
    world.register::<TileTransformable>();
    for y in 0..(MAP_HEIGHT as usize) {
        for x in 0..(MAP_WIDTH as usize) {
            initialise_tile(world, x as f32, y as f32, sprite_store.get_ground());
        }
    }
}
