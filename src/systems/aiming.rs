extern crate amethyst;

use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::Hook;

fn fire_hook(transform: &mut Transform, hook: &mut Hook, mouse_position: (f64, f64)) {
    // TODO: some magic to map mouse coords to screen coords!
    transform.set_xyz(
        mouse_position.0 as f32,
        mouse_position.1 as f32,
        transform.translation().z,
    );
    println!(
        "launch ye hook matey: {}, {}",
        mouse_position.0, mouse_position.1
    )
}

pub struct AimingSystem;

impl<'s> System<'s> for AimingSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hook>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, mut hooks, input): Self::SystemData) {
        for (_hook, _transform) in (&mut hooks, &mut transforms).join() {
            match input.action_is_down("fire") {
                Some(_v) => {
                    if _v {
                        if let Some(position) = input.mouse_position() {
                            fire_hook(_transform, _hook, position);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
