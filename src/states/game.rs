extern crate amethyst;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;

use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
    Texture, TextureMetadata,
};

use crate::components;
use crate::config::ArenaConfig;
use crate::tile_loader;
use crate::util::GameAssets;

pub const GRID_COLS: usize = 32;
pub const GRID_ROWS: usize = 18;

pub const HOOK_RADIUS: f32 = 2.0;
pub const HOOK_DISTANCE: f32 = 60.0;
pub const HOOK_SPEED: f32 = 120.0;

pub const PLAYER_HEIGHT: f32 = 5.0;
pub const PLAYER_WIDTH: f32 = 7.5;
pub const PLAYER_MAX_VELOCITY: [f32; 2] = [100.0, 100.0];
pub const PLAYER_ACCELERATION: f32 = 350.0;

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<components::Tile>();

        let tile_sheet_handle = load_sprite_sheet(world, "texture/tilesheet");
        let sprite_sheet_handle = load_sprite_sheet(world, "texture/hook_spritesheet");
        let assets = GameAssets {
            entities_sprite_sheet: sprite_sheet_handle.clone(),
            tile_sheet: tile_sheet_handle.clone(),
        };

        match tile_loader::load_tile_data(GRID_COLS, GRID_ROWS) {
            Some(data) => tile_loader::populate_world(world, data, assets.clone()),
            _ => {
                panic!("failed to load the tile data");
            }
        };

        world.add_resource(assets);

        initialise_player(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

fn initialise_player(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let (arena_height, arena_width) = {
        let config = &world.read_resource::<ArenaConfig>();
        (config.height, config.width)
    };

    let mut local_transform = Transform::default();
    local_transform.set_xyz(arena_width / 2.0, arena_height / 2.0, 0.0);

    world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: sprite_sheet,
            sprite_number: 1,
        })
        .with(local_transform)
        .with(components::Player {
            velocity: [0.0, 0.0],
            max_velocity: PLAYER_MAX_VELOCITY,
            acceleration: PLAYER_ACCELERATION,
            height: PLAYER_HEIGHT,
            width: PLAYER_WIDTH,
        })
        .with(components::Jump::default())
        .with(components::Gravity)
        .build();
}

fn initialise_camera(world: &mut World) {
    let (arena_height, arena_width) = {
        let config = &world.read_resource::<ArenaConfig>();
        (config.height, config.width)
    };
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            arena_width,
            0.0,
            arena_height,
        )))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World, file_name: &str) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", file_name),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}.ron", file_name), // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}
