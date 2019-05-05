use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};
use super::super::actions::TileTranslation;


pub const PIXELS_PER_TILE: f32 = 32.;

pub struct TileTransformable {
    x: u8,
    y: u8,
    pub moved: bool,
}

impl TileTransformable {
    pub fn new(x: u8, y: u8) -> TileTransformable {
        TileTransformable {
            x,
            y,
            moved: true,
        }
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }
    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn set_x(&mut self, new_x: u8) {
        if self.x != new_x {
            self.x = new_x;
            self.moved = true;
        }
    }
    pub fn set_y(&mut self, new_y: u8) {
        if self.y != new_y {
            self.y = new_y;
            self.moved = true;
        }
    }

    pub fn set_xy(&mut self, new_x: u8, new_y: u8) {
        self.set_x(new_x);
        self.set_y(new_y);
    }
    pub fn get_xy(&mut self) -> (u8,u8){
        (self.get_x(), self.get_y())
    }

    pub fn translate(&mut self, direction: TileTranslation) {
        match direction {
            TileTranslation::UP(dist) => self.set_y(self.get_y() + dist),
            TileTranslation::DOWN(dist) => self.set_y(self.get_y() - dist),
            TileTranslation::RIGHT(dist) => self.set_x(self.get_x() + dist),
            TileTranslation::LEFT(dist) => self.set_x(self.get_x() - dist),
        };
    }


    pub fn get_x_px(&self) -> f32 {
        self.x as f32 * PIXELS_PER_TILE + PIXELS_PER_TILE/2.
    }

    pub fn get_y_px(&self) -> f32 {
        self.y as f32 * PIXELS_PER_TILE + PIXELS_PER_TILE/2.
    }

}

impl Component for TileTransformable {
    type Storage = DenseVecStorage<Self>;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_creates_tile_correctly() {
        let tile = TileTransformable::new(1,2);

        assert_eq!(1, tile.x);
        assert_eq!(2, tile.y);
        assert!(tile.moved);
    }

    #[test]
    fn pixel_calculations_are_correct() {
        let tile = TileTransformable::new(2,7);

        assert_eq!(2.*PIXELS_PER_TILE+PIXELS_PER_TILE/2., tile.get_x_px());
        assert_eq!(7.*PIXELS_PER_TILE+PIXELS_PER_TILE/2., tile.get_y_px());
    }

    #[test]
    fn setters_set_movable_if_pos_changed() {
        let mut tile = TileTransformable::new(2,7);

        tile.moved = false;
        tile.set_x(3);
        assert_eq!(3, tile.get_x());
        assert!(tile.moved);

        tile.moved = false;
        tile.set_y(8);
        assert_eq!(8, tile.get_y());
        assert!(tile.moved);

        tile.moved = false;
        tile.set_xy(4,5);
        assert_eq!(4, tile.get_x());
        assert_eq!(5, tile.get_y());
        assert!(tile.moved);
    }

    #[test]
    fn setters_ignore_movable_when_same_position() {
        let mut tile = TileTransformable::new(2,7);
        tile.moved = false;

        tile.set_x(2);
        assert!(!tile.moved);

        tile.set_y(7);
        assert!(!tile.moved);

        tile.set_xy(2,7);
        assert!(!tile.moved);
    }

    #[test]
    fn translate_moves_correctly() {
        let mut tile = TileTransformable::new(0,0);

        tile.translate(TileTranslation::UP(2));
        assert_eq!(2, tile.get_y());

        tile.translate(TileTranslation::DOWN(2));
        assert_eq!(0, tile.get_x());

        tile.translate(TileTranslation::RIGHT(2));
        assert_eq!(2, tile.get_x());

        tile.translate(TileTranslation::LEFT(2));
        assert_eq!(0, tile.get_x());
    }
}
