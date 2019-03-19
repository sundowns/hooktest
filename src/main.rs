extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA},
    utils::application_root_dir,
};

mod hooktest;
use crate::hooktest::HookTest;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0, 0.4862, 0.4, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?;

    let mut game = Application::new("./", HookTest, game_data)?;

    game.run();

    Ok(())
}
