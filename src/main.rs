extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA},
    utils::application_root_dir,
};

mod components;
mod config;
mod states;
mod systems;
mod tile_loader;
mod util;
use crate::config::ArenaConfig;
use crate::states::Game;

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
        .start();

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.078, 0.2, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let config_path = format!("{}/resources/config.ron", application_root_dir());

    let arena_config = ArenaConfig::load(&config_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            systems::MovementSystem,
            "movement_system",
            &["input_system"],
        )
        .with(systems::JumpingSystem, "jumping_system", &["input_system"])
        .with(
            systems::SpawnHookSystem,
            "spawn_hook_system",
            &["input_system"],
        )
        .with(systems::GravitySystem, "gravity_system", &[])
        .with(systems::MoveHookSystem, "move_hook_system", &[]);

    let mut game = Application::build("./", Game)?
        .with_resource(arena_config)
        .build(game_data)?;

    game.run();

    Ok(())
}
