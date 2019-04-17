use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, System, WriteStorage},
};

use crate::components::Extending;
use crate::components::Hook;

pub struct MoveHookSystem;

impl<'s> System<'s> for MoveHookSystem {
    type SystemData = (
        WriteStorage<'s, Hook>,
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Extending>,
    );

    fn run(&mut self, (mut hooks, time, mut locals, mut is_extending): Self::SystemData) {
        // Move every hook according to its speed, and the time passed.
        for (_hook, local, _is_extending) in (&mut hooks, &mut locals, &mut is_extending).join() {
            local.translate_x(_hook.velocity[0] * time.delta_seconds());
            local.translate_y(_hook.velocity[1] * time.delta_seconds());

            _is_extending.distance_traveled =
                _is_extending.distance_traveled + (_hook.speed * time.delta_seconds());

            if _is_extending.distance_traveled > _hook.max_distance {
                _hook.velocity = vec![0.0, 0.0];
                // TODO: Remove the is_extending component. Do I need to join with entities to achieve this?
            };
        }
    }
}
