use amethyst::prelude::*;

use super::factories;

pub struct MapState;

impl SimpleState for MapState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        factories::map::initialise_map(world);
    }
}
