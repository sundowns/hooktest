extern crate amethyst;

use amethyst::core::{Time, Transform};
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::Player;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut players, input, time): Self::SystemData) {
        for (_player, _transform) in (&mut players, &mut transforms).join() {
            match input.axis_value("horizontal") {
                Some(v) => {
                    let current_velocity_y = _player.velocity[0];
                    _player.velocity[0] = (current_velocity_y
                        + (v as f32 * _player.acceleration * time.delta_seconds()))
                    // .min(-1.0 * _player.max_velocity[0])
                    // .max(1.0 * _player.max_velocity[0]);
                }
                _ => {}
            }

            match input.axis_value("vertical") {
                Some(v) => {
                    let current_velocity_x = _player.velocity[1];
                    _player.velocity[1] = (current_velocity_x
                        + (v as f32 * _player.acceleration * time.delta_seconds()))
                    // .min(-1.0 * _player.max_velocity[1])
                    // .max(1.0 * _player.max_velocity[1]);
                }
                _ => {}
            }

            // TODO: why are my mins/max's fucking me and forcing it to the maximum?

            _transform.translate_x(_player.velocity[0] * time.delta_seconds());
            _transform.translate_y(_player.velocity[1] * time.delta_seconds());
            println!("vel {}, {}", _player.velocity[0], _player.velocity[1]);
        }
    }
}
