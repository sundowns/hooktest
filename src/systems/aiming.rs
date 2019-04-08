extern crate amethyst;

use amethyst::core::Transform;
use amethyst::ecs::{
    Entities, Join, Read, ReadExpect, ReadStorage, Resources, System, WriteStorage,
};
use amethyst::input::InputHandler;
use amethyst::renderer::{ScreenDimensions, SpriteRender, SpriteSheetHandle};

use crate::components::{Hook, Player};
use crate::hookarena::{GameAssets, ARENA_HEIGHT, ARENA_WIDTH, HOOK_RADIUS};

pub struct AimingSystem;

impl<'s> System<'s> for AimingSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hook>,
        ReadStorage<'s, Player>,
        Entities<'s>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, ScreenDimensions>,
        ReadExpect<'s, GameAssets>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (mut transforms, mut hooks, players, entities, input, screen, assets, mut sprites): Self::SystemData,
    ) {
        let screen_ratios = vec![ARENA_WIDTH / screen.width(), ARENA_HEIGHT / screen.height()];
        let mut new_hooks: Vec<Transform> = Vec::new();

        for (_player, _transform) in (&players, &mut transforms).join() {
            match input.action_is_down("fire") {
                Some(_v) => {
                    if _v {
                        if let Some(position) = input.mouse_position() {
                            let mut local_transform = Transform::default();
                            local_transform.set_xyz(
                                ((position.0 as f32) * screen_ratios[0])
                                    .min(ARENA_WIDTH)
                                    .max(0.0),
                                (ARENA_HEIGHT - ((position.1 as f32) * screen_ratios[1]))
                                    .min(ARENA_HEIGHT)
                                    .max(0.0),
                                0.0,
                            );
                            new_hooks.push(local_transform);
                        }
                    }
                }
                _ => {}
            }
        }

        while let Some(_transform) = new_hooks.pop() {
            // TODO: Prevent more than 1 hook being fired at a time per player
            entities
                .build_entity()
                .with(_transform, &mut transforms)
                .with(
                    Hook {
                        velocity: [0.0, 0.0],
                        radius: HOOK_RADIUS,
                    },
                    &mut hooks,
                )
                .with(assets.entity_sprite(0), &mut sprites)
                .build();
        }
    }
}
