extern crate amethyst;

use amethyst::core::Transform;
use amethyst::ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::{ScreenDimensions, SpriteRender};

use crate::components::{Extending, Hook, HookFired, Player};
use crate::hookarena::{GameAssets, ARENA_HEIGHT, ARENA_WIDTH, HOOK_RADIUS};

pub struct NewHook {
    owner: Entity,
    transform: Transform,
    velocity: Vec<f32>,
}

pub struct SpawnHookSystem;

impl<'s> System<'s> for SpawnHookSystem {
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
            screen,
            assets,
            mut sprites,
            mut hooks_fired,
            mut is_extending,
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
                        if let Some(mouse_position) = input.mouse_position() {
                            let world_position = vec![
                                (mouse_position.0 as f32) * screen_ratios[0],
                                (mouse_position.1 as f32) * screen_ratios[1],
                            ];

                            // Y coordinate is subtracted from arena height. Origin is the bottom left
                            // TODO: unit vector * hook speed
                            let vel = vec![
                                ((world_position[0] as f32) - _transform.translation().x),
                                ((ARENA_HEIGHT - (world_position[1] as f32))
                                    - _transform.translation().y),
                            ];

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

            println!("{:?}", _new_hook.velocity);

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
