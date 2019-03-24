extern crate amethyst;

use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::ScreenDimensions;

use crate::components::Hook;
use crate::hookarena::{ARENA_HEIGHT, ARENA_WIDTH};

fn fire_hook(transform: &mut Transform, mouse_position: (f64, f64), screen_ratios: &Vec<f32>) {
    transform.set_xyz(
        ((mouse_position.0 as f32) * screen_ratios[0])
            .min(ARENA_WIDTH)
            .max(0.0),
        (ARENA_HEIGHT - ((mouse_position.1 as f32) * screen_ratios[1]))
            .min(ARENA_HEIGHT)
            .max(0.0),
        transform.translation().z,
    );
}

pub struct AimingSystem;

impl<'s> System<'s> for AimingSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hook>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut transforms, mut hooks, input, screen): Self::SystemData) {
        let screen_ratios = vec![ARENA_WIDTH / screen.width(), ARENA_HEIGHT / screen.height()];

        for (_hook, _transform) in (&mut hooks, &mut transforms).join() {
            match input.action_is_down("fire") {
                Some(_v) => {
                    if _v {
                        if let Some(position) = input.mouse_position() {
                            fire_hook(_transform, position, &screen_ratios);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
