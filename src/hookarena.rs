extern crate amethyst;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;

use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
    Texture, TextureMetadata,
};

use crate::components;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const HOOK_RADIUS: f32 = 2.0;

pub struct HookArena;

impl SimpleState for HookArena {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<components::Hook>();
        world.register::<components::Player>();

        initialise_player(world, sprite_sheet_handle.clone());
        initialise_hook(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

fn initialise_player(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut local_transform = Transform::default();
    local_transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // Assign the sprite
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(components::Player {
            velocity: [0.0, 0.0],
            max_velocity: [30.0, 30.0],
            acceleration: 50.0,
        })
        .build();
}

fn initialise_hook(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut local_transform = Transform::default();
    local_transform.set_xyz(ARENA_WIDTH / 2.0 + 20.0, ARENA_HEIGHT / 2.0 + 20.0, 0.0); // TODO: hacked atm to move out of the way (+20)s

    // Assign the sprite
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(components::Hook {
            velocity: [0.0, 0.0],
            radius: HOOK_RADIUS,
        })
        .with(local_transform)
        .build();
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
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
