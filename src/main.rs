extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{
        DisplayConfig,
        DrawFlat2D,
        Pipeline,
        RenderBundle,
        Stage,
        ColorMask,
        ALPHA,
    },
    utils::application_root_dir,
    core::transform::TransformBundle,
};

mod game;
use game::states;

use game::tile_map::systems as tile_map_systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let root_dir = application_root_dir();

    let path = format!("{}/resources/display_config.ron", root_dir,);
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None))
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with(tile_map_systems::SyncTileTransformable, "sync_tile_transformable", &[]);

    let mut game = Application::new("./", states::MapState, game_data)?;

    game.run();

    Ok(())
}
