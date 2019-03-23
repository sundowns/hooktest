extern crate amethyst;

use amethyst::core::{Time, Transform};
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::Player;
use crate::hookarena::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct MovementSystem;

pub const FRICTION: f32 = 0.3;

fn apply_friction(player: &mut Player, delta_time: f32) {
    // Apply friction and cap velocities
    player.velocity[0] = player.velocity[0]
        - (player.velocity[0] * 1.0 / FRICTION * delta_time)
            .min(player.max_velocity[0])
            .max(-1.0 * player.max_velocity[0]);

    player.velocity[1] = player.velocity[1]
        - (player.velocity[1] * 1.0 / FRICTION * delta_time)
            .min(player.max_velocity[1])
            .max(-1.0 * player.max_velocity[1]);
}

fn apply_translations(player: &mut Player, transform: &mut Transform, delta_time: f32) {
    // Apply translations
    let player_x = transform.translation().x;
    transform.set_x(
        (player_x + (player.velocity[0] * delta_time))
            .min(ARENA_WIDTH)
            .max(0.0),
    );

    let player_y = transform.translation().y;
    transform.set_y(
        (player_y + (player.velocity[1] * delta_time))
            .min(ARENA_HEIGHT)
            .max(0.0),
    );
}

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

            apply_friction(_player, time.delta_seconds());
            apply_translations(_player, _transform, time.delta_seconds());
        }
    }
}
