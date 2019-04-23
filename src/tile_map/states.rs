use amethyst::prelude::*;

use super::factories;
use super::resources::sprites::SpriteStore;

pub struct MapState {
    sprite_store: SpriteStore
}


impl SimpleState for MapState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;


        factories::map::initialise_map(world);
    }
}
