extern crate amethyst;

use amethyst::core::Transform;
use amethyst::ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::{ScreenDimensions, SpriteRender};

use crate::components::{Extending, Hook, HookFired, Player};
use crate::config::ArenaConfig;
use crate::hookarena::{GameAssets, HOOK_DISTANCE, HOOK_RADIUS, HOOK_SPEED};

pub struct NewHook {
    owner: Entity,
    transform: Transform,
    velocity: [f32; 2],
}

pub struct SpawnHookSystem;

impl<'s> System<'s> for SpawnHookSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hook>,
        ReadStorage<'s, Player>,
        Entities<'s>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, ArenaConfig>,
        ReadExpect<'s, ScreenDimensions>,
        ReadExpect<'s, GameAssets>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, HookFired>,
        WriteStorage<'s, Extending>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            mut hooks,
            players,
            entities,
            input,
            arena_config,
            screen,
            assets,
            mut sprites,
            mut hooks_fired,
            mut is_extending,
        ): Self::SystemData,
    ) {
        let screen_ratios = vec![
            arena_config.width / screen.width(),
            arena_config.height / screen.height(),
        ];
        let mut new_hooks: Vec<NewHook> = Vec::new();

        for (_entity, _player, _transform, _) in
            (&entities, &players, &mut transforms, !&hooks_fired).join()
        {
            match input.action_is_down("fire") {
                Some(_is_down) => {
                    if _is_down {
                        if let Some(mouse_position) = input.mouse_position() {
                            let world_position = vec![
                                (mouse_position.0 as f32) * screen_ratios[0],
                                (mouse_position.1 as f32) * screen_ratios[1],
                            ];

                            // Y coordinate is subtracted from arena height. Origin is the bottom left
                            let mut vel = [
                                ((world_position[0] as f32) - _transform.translation().x),
                                ((arena_config.height - (world_position[1] as f32))
                                    - _transform.translation().y),
                            ];

                            let magnitude = ((vel[0] * vel[0]) + (vel[1] * vel[1])).sqrt();

                            // Normalise hook speeds
                            vel[0] = vel[0] / magnitude * HOOK_SPEED;
                            vel[1] = vel[1] / magnitude * HOOK_SPEED;

                            new_hooks.push(NewHook {
                                velocity: vel,
                                transform: _transform.clone(),
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
                        max_distance: HOOK_DISTANCE,
                        speed: HOOK_SPEED,
                        owner: _new_hook.owner,
                    },
                    &mut hooks,
                )
                .with(
                    Extending {
                        distance_traveled: 0.0,
                    },
                    &mut is_extending,
                )
                .with(assets.entity_sprite(0), &mut sprites)
                .build();
        }
    }
}
