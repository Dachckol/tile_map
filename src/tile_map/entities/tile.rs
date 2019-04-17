use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub const PIXELS_PER_TILE: usize = 16;

pub enum TileType {
    GROUND,
    WALL,
    TREE,
}

pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub tile_type: Option<TileType>,
}

impl Tile {
    pub fn new_empty(x: usize, y: usize) -> Tile {
        Tile {
            x,
            y,
            tile_type: Option::None,
        }
    }

    pub fn new_with_type(x: usize, y: usize, tile_type: TileType) -> Tile {
        Tile {
            x,
            y,
            tile_type: Option::Some(tile_type),
        }
    }


    pub fn is_empty(&self) -> bool {
        return self.tile_type.is_none()
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
    fn new_empty_creates_empty_tile() {
        let tile = Tile::new_empty(1,2);

        assert_eq!(1, tile.x);
        assert_eq!(2, tile.y);
        assert!(tile.is_empty());
    }


    #[test]
    fn new_with_type_correctly_creates_tile() {
        let tile = Tile::new_with_type(1,2, TileType::GROUND);

        assert_eq!(1, tile.x);
        assert_eq!(2, tile.y);
        assert!(!tile.is_empty());
    }

    #[test]
    fn pixel_calculations_are_correct() {
        let tile = Tile::new_empty(2,7);

        assert_eq!(2*PIXELS_PER_TILE, tile.get_x_px());
        assert_eq!(7*PIXELS_PER_TILE, tile.get_y_px());
    }
}
