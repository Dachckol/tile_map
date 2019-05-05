use amethyst::{
    prelude::World,
    renderer::{SpriteRender, SpriteSheetHandle},
};

use super::loaders::load_sprite_sheet;

pub struct SpriteStore {
    sheet_handle: SpriteSheetHandle,
}

impl SpriteStore {
    pub fn new(world: &mut World) -> SpriteStore {
        SpriteStore {
            sheet_handle: load_sprite_sheet(world),
        }
    }

    pub fn get_ground(&self) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.sheet_handle.clone(),
            sprite_number: 0,
        }
    }

    pub fn get_player(&self) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.sheet_handle.clone(),
            sprite_number: 1,
        }
    }
}
