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

pub const HOOK_RADIUS: f32 = 2.0;
pub const HOOK_DISTANCE: f32 = 100.0;
pub const HOOK_SPEED: f32 = 40.0;

#[derive(Clone)]
pub struct GameAssets {
    entities_sprite_sheet: SpriteSheetHandle,
}

impl GameAssets {
    pub fn entity_sprite(&self, sprite_number: usize) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.entities_sprite_sheet.clone(),
            sprite_number,
        }
    }
}

pub struct HookArena;

impl SimpleState for HookArena {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);
        world.add_resource(GameAssets {
            entities_sprite_sheet: sprite_sheet_handle.clone(),
        });

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
            max_velocity: [100.0, 100.0],
            acceleration: 500.0,
        })
        // .with(components::Gravity)
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

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/hook_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/hook_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}
