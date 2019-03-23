extern crate amethyst;

use amethyst::core::{Time, Transform};
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::Player;
use crate::hookarena::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct MovementSystem;

pub const FRICTION: f32 = 0.3;

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
                    _player.velocity[0] = current_velocity_y
                        + (v as f32 * _player.acceleration * time.delta_seconds());
                }
                _ => {}
            }

            match input.axis_value("vertical") {
                Some(v) => {
                    let current_velocity_x = _player.velocity[1];
                    _player.velocity[1] = current_velocity_x
                        + (v as f32 * _player.acceleration * time.delta_seconds());
                }
                _ => {}
            }

            // Apply friction and cap velocities
            _player.velocity[0] = (_player.velocity[0]
                - (_player.velocity[0] * 1.0 / FRICTION * time.delta_seconds()))
            .min(_player.max_velocity[0])
            .max(-1.0 * _player.max_velocity[0]);

            _player.velocity[1] = _player.velocity[1]
                - (_player.velocity[1] * 1.0 / FRICTION * time.delta_seconds())
                    .min(_player.max_velocity[1])
                    .max(-1.0 * _player.max_velocity[1]);

            // Apply translations
            let player_x = _transform.translation().x;
            _transform.set_x(
                (player_x + (_player.velocity[0] * time.delta_seconds()))
                    .min(ARENA_WIDTH)
                    .max(0.0),
            );

            let player_y = _transform.translation().y;
            _transform.set_y(
                (player_y + (_player.velocity[1] * time.delta_seconds()))
                    .min(ARENA_HEIGHT)
                    .max(0.0),
            );
        }
    }
}
