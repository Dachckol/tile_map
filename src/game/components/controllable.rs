use amethyst::{
    ecs::{
        Component,
        DenseVecStorage,
    },
};
use crate::game::tile_map::actions::TileTranslation;

pub struct Controllable {
    bindings: Vec<(String, TileTranslation)>,
}


impl Component for Controllable {
    type Storage = DenseVecStorage<Self>;
}


impl Controllable {
    pub fn new(bindings: Vec<(String, TileTranslation)>) -> Controllable {
        Controllable{bindings}
    }

    pub fn get_action(&self, name: &str) -> Option<TileTranslation> {
        for (input_label, action) in &self.bindings {
            if input_label == name {
                return Some(action.clone());
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn able_to_set_and_retrieve_bindings() {
        let controllable = Controllable::new(
                vec![
                    (String::from("up_label"), TileTranslation::UP(2)),
                    (String::from("down_label"), TileTranslation::DOWN(3)),
                ]
            );

        assert!(same_enum!(controllable.get_action("up_label"), Some(TileTranslation::UP(2))));
        assert!(same_enum!(controllable.get_action("down_label"), Some(TileTranslation::DOWN(3))));
    }

    #[test]
    fn returns_none_for_no_action() {
        let controllable = Controllable::new(
                vec![
                    (String::from("up_label"), TileTranslation::UP(2)),
                ]
            );

        assert!(same_enum!(controllable.get_action("something_else"), None));
    }
}
