use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};

pub const PIXELS_PER_TILE: f32 = 32.;

pub struct TileTransformable {
    pub x: f32,
    pub y: f32,
}

impl TileTransformable {
    pub fn new(x: f32, y: f32) -> TileTransformable {
        TileTransformable {
            x,
            y,
        }
    }

    pub fn get_x_px(&self) -> f32 {
        self.x * PIXELS_PER_TILE + PIXELS_PER_TILE/2 as f32
    }

    pub fn get_y_px(&self) -> f32 {
        self.y * PIXELS_PER_TILE + PIXELS_PER_TILE/2 as f32
    }

//    pub fn sync_transform(&self) -> 
}

impl Component for TileTransformable {
    type Storage = DenseVecStorage<Self>;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_creates_tile_correctly() {
        let tile = TileTransformable::new(1.,2.);

        assert_eq!(1., tile.x);
        assert_eq!(2., tile.y);
    }

    #[test]
    fn pixel_calculations_are_correct() {
        let tile = TileTransformable::new(2.,7.);

        assert_eq!(2.*PIXELS_PER_TILE+PIXELS_PER_TILE/2., tile.get_x_px());
        assert_eq!(7.*PIXELS_PER_TILE+PIXELS_PER_TILE/2., tile.get_y_px());
    }
}