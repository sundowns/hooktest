extern crate amethyst;

use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, Read, ReadExpect, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::ScreenDimensions;
// use amethyst::specs::world::EntitiesRes;

use crate::components::{Hook, Player};
use crate::hookarena::{ARENA_HEIGHT, ARENA_WIDTH, HOOK_RADIUS};

pub struct AimingSystem;

impl<'s> System<'s> for AimingSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Hook>,
        Entities<'s>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (mut transforms, mut players, mut hooks, entities, input, screen): Self::SystemData,
    ) {
        let screen_ratios = vec![ARENA_WIDTH / screen.width(), ARENA_HEIGHT / screen.height()];

        for (_player, _transform) in (&mut players, &mut transforms).join() {
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

                            entities
                                .build_entity()
                                .with(local_transform, &mut transforms)
                                .with(
                                    Hook {
                                        velocity: [0.0, 0.0],
                                        radius: HOOK_RADIUS,
                                    },
                                    &mut hooks,
                                )
                                .build();

                            // TODO: how in the world can I get a sprite for the hook?
                            // self.fire_hook(_transform, position, entities, &screen_ratios);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

// impl AimingSystem {
//     fn fire_hook(
//         &self,
//         origin: &mut Transform,
//         target: (f64, f64),
//         entities: Read<amethyst::EntitiesRes>,
//         screen_ratios: &Vec<f32>,
//     ) {

//         create a new hook at the below
//         transform.set_xyz(
//             ((mouse_position.0 as f32) * screen_ratios[0])
//                 .min(ARENA_WIDTH)
//                 .max(0.0),
//             (ARENA_HEIGHT - ((mouse_position.1 as f32) * screen_ratios[1]))
//                 .min(ARENA_HEIGHT)
//                 .max(0.0),
//             transform.translation().z,
//         );
//     }
// }
