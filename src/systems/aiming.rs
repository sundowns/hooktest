extern crate amethyst;

use amethyst::core::Transform;
use amethyst::ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::{ScreenDimensions, SpriteRender};

use crate::components::{Hook, HookFired, Player};
use crate::hookarena::{GameAssets, ARENA_HEIGHT, ARENA_WIDTH, HOOK_RADIUS};

pub struct NewHook {
    owner: Entity,
    transform: Transform,
    velocity: [f32; 2],
}

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
        WriteStorage<'s, HookFired>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            mut hooks,
            players,
            entities,
            input,
            screen,
            assets,
            mut sprites,
            mut hooks_fired,
        ): Self::SystemData,
    ) {
        let screen_ratios = vec![ARENA_WIDTH / screen.width(), ARENA_HEIGHT / screen.height()];
        let mut new_hooks: Vec<NewHook> = Vec::new();

        for (_entity, _player, _transform, _) in
            (&entities, &players, &mut transforms, !&hooks_fired).join()
        {
            match input.action_is_down("fire") {
                Some(_is_down) => {
                    if _is_down {
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

                            // TODO: spawn hook at player^

                            let x_vel = position.0 - _transform.x; // TTODO: figure out transform api
                            let y_vel = position.1 - _transform.y;

                            new_hooks.push(NewHook {
                                velocity: [x_vel, y_vel],
                                transform: local_transform,
                                owner: _entity,
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        while let Some(_new_hook) = new_hooks.pop() {
            match hooks_fired.insert(_new_hook.owner, HookFired) {
                Err(_v) => panic!("Failed to store new hook"),
                _ => {}
            };

            entities
                .build_entity()
                .with(_new_hook.transform, &mut transforms)
                .with(
                    Hook {
                        velocity: _new_hook.velocity,
                        radius: HOOK_RADIUS,
                    },
                    &mut hooks,
                )
                .with(assets.entity_sprite(0), &mut sprites)
                .build();
        }
    }
}
