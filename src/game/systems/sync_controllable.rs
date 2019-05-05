use amethyst::{
    core::timing::Time,
    ecs::prelude::{Join, Read, System, WriteStorage},
    input::InputHandler,
};

use crate::game::components::Controllable;
use crate::tile_map::components::TileTransformable;

const PULSE: f32 = 0.09;

pub struct SyncControllable {
    last_down: f32,
}

impl SyncControllable {
    pub fn new() -> SyncControllable {
        SyncControllable { last_down: 0. }
    }
}

impl<'s> System<'s> for SyncControllable {
    type SystemData = (
        WriteStorage<'s, Controllable>,
        WriteStorage<'s, TileTransformable>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut controllables, mut tile_transformables, input, time): Self::SystemData) {
        self.last_down += time.delta_seconds();
        for (controllable, tile_transformable) in
            (&mut controllables, &mut tile_transformables).join()
        {
            if self.last_down < PULSE {
                continue;
            }
            let actions = controllable.get_actions();
            for action in actions {
                if let Some(true) = input.action_is_down(action) {
                    tile_transformable.translate(controllable.get_command(action).unwrap());
                    self.last_down = 0.;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
}
