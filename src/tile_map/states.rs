use amethyst::prelude::*;

use super::factories;
use super::resources::sprites::SpriteStore;

pub struct MapState;


impl SimpleState for MapState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_store = SpriteStore::new(world);

        factories::core::initialise_camera(world);
        factories::map::initialise_map(world, &sprite_store);
    }
}
