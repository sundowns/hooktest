extern crate amethyst;

use amethyst::core::{Time, Transform};
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::Player;
use crate::config::ArenaConfig;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
        Read<'s, ArenaConfig>,
    );

    fn run(&mut self, (mut transforms, mut players, input, time, arena_config): Self::SystemData) {
        for (_player, _transform) in (&mut players, &mut transforms).join() {
            match input.axis_value("horizontal") {
                Some(v) => {
                    let current_velocity_y = _player.velocity[0];
                    _player.velocity[0] = current_velocity_y
                        + (v as f32 * _player.acceleration * time.delta_seconds());
                }
                _ => {}
            }

            self.apply_friction(_player, time.delta_seconds(), &arena_config);
            self.apply_translations(_player, _transform, time.delta_seconds(), &arena_config);
        }
    }
}

impl MovementSystem {
    fn apply_friction(&self, player: &mut Player, delta_time: f32, arena_config: &ArenaConfig) {
        // Apply friction and cap velocities
        player.velocity[0] = player.velocity[0]
            - (player.velocity[0] * 1.0 / arena_config.friction * delta_time)
                .min(player.max_velocity[0])
                .max(-1.0 * player.max_velocity[0]);

        player.velocity[1] = player.velocity[1]
            - (player.velocity[1] * 1.0 / arena_config.friction * delta_time)
                .min(player.max_velocity[1])
                .max(-1.0 * player.max_velocity[1]);
    }

    fn apply_translations(
        &self,
        player: &mut Player,
        transform: &mut Transform,
        delta_time: f32,
        arena_config: &ArenaConfig,
    ) {
        // Apply translations
        let player_x = transform.translation().x;
        transform.set_x(
            (player_x + (player.velocity[0] * delta_time))
                .min(arena_config.width - player.width / 2.0)
                .max(0.0 + player.width / 2.0),
        );

        let player_y = transform.translation().y;
        transform.set_y(
            (player_y + (player.velocity[1] * delta_time))
                .min(arena_config.height)
                .max(0.0 + player.height),
        );
    }
}
