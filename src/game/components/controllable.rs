use crate::tile_map::commands::TileTranslation;
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Controllable {
    bindings: Vec<(&'static str, TileTranslation)>,
}

impl Component for Controllable {
    type Storage = DenseVecStorage<Self>;
}

impl Controllable {
    pub fn new(bindings: Vec<(&'static str, TileTranslation)>) -> Controllable {
        Controllable { bindings }
    }

    pub fn get_command(&self, name: &'static str) -> Option<TileTranslation> {
        for (input_label, command) in &self.bindings {
            if *input_label == name {
                return Some(command.clone());
            }
        }
        return None;
    }

    pub fn get_actions(&self) -> Vec<&'static str> {
        let mut actions = Vec::new();
        for (input_label, _) in &self.bindings {
            actions.push(input_label.clone());
        }
        return actions;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn able_to_set_and_retrieve_bindings() {
        let controllable = Controllable::new(vec![
            ("up_label", TileTranslation::UP(2)),
            ("down_label", TileTranslation::DOWN(3)),
        ]);

        assert!(same_enum!(
            controllable.get_command("up_label"),
            Some(TileTranslation::UP(2))
        ));
        assert!(same_enum!(
            controllable.get_command("down_label"),
            Some(TileTranslation::DOWN(3))
        ));
    }

    #[test]
    fn returns_none_for_no_action() {
        let controllable = Controllable::new(vec![("up_label", TileTranslation::UP(2))]);

        assert!(same_enum!(controllable.get_command("something_else"), None));
    }

    #[test]
    fn get_actions_returns_all_actions() {
        let controllable = Controllable::new(vec![
            ("up_label", TileTranslation::UP(2)),
            ("down_label", TileTranslation::DOWN(3)),
        ]);

        let actions = controllable.get_actions();
        assert_eq!(2, actions.len());
        assert!(actions.contains(&"up_label"));
        assert!(actions.contains(&"down_label"));
    }
}
