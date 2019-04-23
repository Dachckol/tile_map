use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub const PIXELS_PER_TILE: usize = 16;

pub struct Tile {
    pub x: usize,
    pub y: usize,
}

impl Tile {
    pub fn new(x: usize, y: usize) -> Tile {
        Tile {
            x,
            y,
        }
    }

    pub fn get_x_px(&self) -> usize {
        self.x * PIXELS_PER_TILE
    }

    pub fn get_y_px(&self) -> usize {
        self.y * PIXELS_PER_TILE
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_tile_correctly() {
        let tile = Tile::new(1,2);

        assert_eq!(1, tile.x);
        assert_eq!(2, tile.y);
    }

    #[test]
    fn pixel_calculations_are_correct() {
        let tile = Tile::new(2,7);

        assert_eq!(2*PIXELS_PER_TILE, tile.get_x_px());
        assert_eq!(7*PIXELS_PER_TILE, tile.get_y_px());
    }
}
